use crate::controller::handle_request::TaxPlotDataResponse;
use crate::core::schedules::marginal_schedule::MarginalIncomeTaxRateSchedule;
use crate::exchange_rates::{
    exchange_rate_adjustment, fetch_exchange_rates, get_currency_country_mapping,
};
use crate::utils::{compute_effective_tax_rates, generate_range};
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
        let file = fs::File::open(config_path).expect("File should open read only");
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
        let exchange_rate_one = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country_one]],
            None => 1.0,
        };
        let exchange_rate_two = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country_two]],
            None => 1.0,
        };
        let schedule_one = &self
            .get_country(country_one)
            .unwrap()
            .to_income_amount_schedule(max_income_to_consider)
            .exchange_rate_adjustment(Some(exchange_rate_one));
        let schedule_two = &self
            .get_country(country_two)
            .unwrap()
            .to_income_amount_schedule(max_income_to_consider)
            .exchange_rate_adjustment(Some(exchange_rate_two));
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
        incomes_to_compute: &[f32],
        max_income: f32,
        specific_income: f32,
        exchange_rate_config: &Option<HashMap<String, f32>>,
        country_currency_mapping: &HashMap<&'static str, &'static str>,
    ) -> TaxData {
        let exchange_rate = match exchange_rate_config {
            Some(exchange_rates) => exchange_rates[country_currency_mapping[country]],
            None => 1.0,
        };
        let schedule = &self
            .get_country(&country)
            .unwrap()
            .to_income_amount_schedule(max_income)
            .exchange_rate_adjustment(Some(exchange_rate));
        let specific_income = specific_income * exchange_rate;
        let incomes_to_compute = exchange_rate_adjustment(&incomes_to_compute, exchange_rate); // guess
        let tax_amounts = match schedule.compute_income_taxes(&incomes_to_compute) {
            Ok(value) => value,
            Err(err) => panic!("Error {:?}", err),
        };
        let effective_tax_rates = compute_effective_tax_rates(&incomes_to_compute, &tax_amounts);

        // Get the specific income
        let specific_tax_amount = schedule
            .compute_specific_income_tax(specific_income)
            .unwrap();
        let specific_tax_rate = specific_tax_amount / specific_income;

        TaxData {
            tax_amounts,
            effective_tax_rates,
            incomes: incomes_to_compute.to_vec(),
            specific_tax_amount: Some(specific_tax_amount),
            specific_tax_rate: Some(specific_tax_rate),
        }
    }

    /// Process the request to compute taxes information
    pub async fn process_request(
        &self,
        req: &TaxPlotDataRequest,
    ) -> Result<TaxPlotDataResponse, String> {
        let step = 10.0;
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
