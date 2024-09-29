use crate::controller::handle_request::TaxPlotDataResponse;
use crate::core::points::marginal_rate_knot::MarginalRateKnot;
use crate::core::schedules::marginal_schedule::MarginalIncomeTaxRateSchedule;
use crate::exchange_rates::{fetch_exchange_rates, get_currency_country_mapping};
use crate::utils::{adjust_exchange_rate_schedule, compute_effective_tax_rates, generate_range};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use super::handle_request::TaxPlotDataRequest;

/// A taxes config represents all information available.
#[derive(Deserialize, Debug, Clone)]
pub struct TaxesConfig {
    /// Mapping from country to its tax schedule.
    pub country_map: HashMap<String, MarginalIncomeTaxRateSchedule>,
}
impl TaxesConfig {
    pub fn new(config_path: &str) -> TaxesConfig {
        let file = fs::File::open(config_path)
            .expect(format!("File should open read only, reading {}", config_path).as_str());
        let json: TaxesConfig = serde_json::from_reader(file).expect("JSON was not well formatted");
        json
    }
    pub fn get_country(&self, country: &str) -> Option<&MarginalIncomeTaxRateSchedule> {
        self.country_map.get(country)
    }

    /// Process breakeven points
    fn process_country_breakeven_points(
        &self,
        country_one: &str,
        country_two: &str,
        max_income_to_consider: f32,
        exchange_rate_config: &Option<HashMap<String, f32>>,
        country_currency_mapping: &HashMap<&'static str, &'static str>,
    ) -> BreakevenData {
        // TODO: Handle exchange rates in a cleaner way
        let exchange_rate_one = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country_one]],
            None => 1.0,
        };
        let exchange_rate_two = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country_two]],
            None => 1.0,
        };
        let schedule_one = adjust_exchange_rate_schedule(
            &self,
            &country_one,
            &Some(exchange_rate_one),
            max_income_to_consider,
        );
        let schedule_two = adjust_exchange_rate_schedule(
            &self,
            &country_two,
            &Some(exchange_rate_two),
            max_income_to_consider,
        );
        let breakevens = schedule_one.compute_breakeven_taxes(&schedule_two);
        let (breakeven_incomes, breakeven_amounts): (Vec<f32>, Vec<f32>) = breakevens
            .par_iter()
            // The origin is not interesting, so filter it out
            .filter(|point| !(point.income() == 0.0 && point.income_tax_amount() == 0.0))
            .map(|point| (point.income(), point.income_tax_amount()))
            .unzip();

        BreakevenData {
            breakeven_incomes: breakeven_incomes.clone(),
            breakeven_tax_amounts: breakeven_amounts.clone(),
            breakeven_effective_tax_rates: compute_effective_tax_rates(
                &breakeven_incomes,
                &breakeven_amounts,
            ),
        }
    }

    /// Process taxes for a country
    fn process_country_taxes(
        &self,
        country: &str,
        currency: &Option<String>, // kind of a hack
        incomes_to_compute: &[f32],
        max_income: f32,
        specific_income: Option<f32>,
        exchange_rate_config: &Option<HashMap<String, f32>>, // feels like a hack having base
        // currency there too.
        country_currency_mapping: &HashMap<&'static str, &'static str>,
    ) -> TaxData {
        let exchange_rate = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country]],
            None => 1.0,
        };
        // TODO: We can move this to somewhere else not utils
        let schedule =
            adjust_exchange_rate_schedule(&self, &country, &Some(exchange_rate), max_income);
        let tax_amounts = match schedule.compute_income_taxes(&incomes_to_compute) {
            Ok(value) => value,
            Err(err) => panic!("Error {:?}", err),
        };
        let effective_tax_rates = compute_effective_tax_rates(&incomes_to_compute, &tax_amounts);

        // Get the specific income
        let specific_tax_amount = schedule.compute_specific_income_tax(specific_income);
        let specific_tax_rate = specific_tax_amount.and_then(|tax_amount| {
            specific_income.map(|specific_income| {
                if specific_income != 0.0 {
                    tax_amount / specific_income
                } else {
                    0.0
                }
            })
        });

        TaxData {
            tax_amounts,
            effective_tax_rates,
            // we want to pass back the income so that the plots that use income on client side
            // is always synced up with the backend "compute" income.
            specific_income,
            specific_tax_amount,
            specific_tax_rate,
            currency: currency.clone(),
            incomes: incomes_to_compute.to_vec(),
            tax_brackets: self.get_country(country).unwrap().schedule().to_vec(),
            exchange_rate: if exchange_rate == 1.0 {
                None
            } else {
                Some(exchange_rate)
            },
        }
    }

    /// Process the request to compute taxes information
    pub async fn process_request(
        &self,
        req: &TaxPlotDataRequest,
    ) -> Result<TaxPlotDataResponse, String> {
        let step = if req.max_income < 1e6 {10.0} else {100.0}; // simple adaptive step size for
        // speedup
        let min_income = 0.0;
        let incomes_to_compute = generate_range(min_income, req.max_income, step);
        let country_currency_mapping = get_currency_country_mapping();
        let exchange_rates_config = match &req.normalizing_currency {
            Some(currency) => Some(fetch_exchange_rates(&currency).await.unwrap()),
            None => None,
        };
        let country_specific_data: HashMap<String, TaxData> = req
            .countries
            .par_iter()
            .map(|country| {
                let tax_data = self.process_country_taxes(
                    &country,
                    &req.normalizing_currency,
                    &incomes_to_compute,
                    req.max_income,
                    req.income,
                    &exchange_rates_config,
                    &country_currency_mapping,
                );
                (country.clone(), tax_data)
            })
            .collect();

        let mut country_comb_data = HashMap::new();
        if req.show_break_even {
            country_comb_data = req
                .countries
                .par_iter()
                .enumerate()
                .flat_map(|(i, country_i)| {
                    req.countries[i + 1..].par_iter().map({
                        let exchange_rates_config = exchange_rates_config.clone();
                        let country_currency_mapping = country_currency_mapping.clone();
                        move |country_j| {
                            let comb_data = self.process_country_breakeven_points(
                                country_i,
                                country_j,
                                req.max_income,
                                &exchange_rates_config,
                                &country_currency_mapping,
                            );
                            (format!("{}-{}", country_i, country_j), comb_data)
                        }
                    })
                })
                .collect();
        }

        Ok(TaxPlotDataResponse {
            country_specific_data,
            country_comb_data: if req.show_break_even {
                Some(country_comb_data)
            } else {
                None
            },
        })
    }
}

// Other structs linked to TaxesConfig
#[derive(Serialize)]
pub struct BreakevenData {
    pub breakeven_incomes: Vec<f32>,
    pub breakeven_tax_amounts: Vec<f32>,
    pub breakeven_effective_tax_rates: Vec<f32>,
}

#[derive(Serialize)]
pub struct TaxData {
    pub incomes: Vec<f32>,
    pub tax_amounts: Vec<f32>, // TODO: tax amounts not needed can just use knot points.
    pub effective_tax_rates: Vec<f32>,
    pub specific_tax_amount: Option<f32>,
    pub specific_tax_rate: Option<f32>,
    pub tax_brackets: Vec<MarginalRateKnot>,
    pub exchange_rate: Option<f32>,
    pub specific_income: Option<f32>,
    pub currency: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::controller::taxes_config::TaxesConfig;
    #[test]
    fn test_taxes_config() {
        let file_path = "test_data/valid_config.json";
        let taxes_config = TaxesConfig::new(&file_path);

        assert_eq!(taxes_config.country_map.len(), 2);
        assert!(taxes_config.country_map.contains_key("New Zealand"));
        assert!(taxes_config.country_map.contains_key("Australia"));

        assert_eq!(
            taxes_config
                .country_map
                .get("New Zealand")
                .unwrap()
                .schedule()
                .len(),
            5
        );
        assert_eq!(
            taxes_config
                .country_map
                .get("Australia")
                .unwrap()
                .schedule()
                .len(),
            5
        );
    }
}
