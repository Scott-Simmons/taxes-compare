mod errors;
use log::info;
use env_logger;
use serde::Deserializer;
mod models;
mod utils;
use crate::models::request::TaxRequest;
use crate::models::response::TaxData;
use errors::errors::TaxError; // TODO: Organise better
use models::{
    response::{BreakevenData, TaxResponse},
    segment::LinearPiecewiseSegment,
    tax_schedule::{compute_breakeven_points, interpolate_segments_parallel, IncomeTaxSchedule},
    taxes_config::TaxesConfig,
};
use std::collections::HashMap;
use utils::utils::generate_range; // TODO: Organise better

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rayon::prelude::*;
use serde::Deserialize;

/// A point characterised by a marginal tax rate at a given level of income
#[derive(Clone, Debug, Deserialize)]
struct MarginalRateKnot {
    /// The marginal tax rate f(x) at given income threshold x
    marginal_rate: f32,
    /// The income threshold at which the knot is the boundry point
    #[serde(deserialize_with = "null_to_infinity")]
    income_limit: Option<f32>, // unbounded at the last entry
}

/// A point characterised by tax amount at given income, which is also denoted as a knot point
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct IncomeTaxKnot {
    /// Income tax amount f(x) for a given maximimum income level x
    income_tax_amount: f32,
    /// The income threshold at which the knot acts as the boundry point
    income_limit: f32,
}

/// A point characterised by tax amount at a given income
#[derive(Debug, PartialEq, Clone)]
struct IncomeTaxPoint {
    /// Income tax amount f(x) for given level of income x
    income_tax_amount: f32,
    /// Level of income x
    income: f32,
}

fn null_to_infinity<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let option: Option<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    match option {
        Some(serde_json::Value::Null) => Ok(Some(f32::INFINITY)),
        Some(serde_json::Value::Number(num)) => num
            .as_f64()
            .map(|f| Some(f as f32))
            .ok_or_else(|| serde::de::Error::custom("Invalid number format")),
        None => Ok(None),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

/// Given the tax amounts and the incomes, compute the effective tax rate at each income step.
fn compute_effective_tax_rates(incomes: &[f32], income_tax_amounts: &[f32]) -> Vec<f32> {
    // Consider par_iter() vs iter(). Depends on the step size where mc = mr. (thread management
    // overhead cost is a consideration)
    incomes
        .par_iter()
        .zip(income_tax_amounts.par_iter())
        .map(|(&income, &income_amount)| {
            if income == 0.0 {
                0.0 // avoid div by zero error
            } else {
                income_amount / income
            }
        })
        .collect()
}

async fn handle_request(
    req: web::Json<TaxRequest>,
    config: web::Data<TaxesConfig>,
) -> impl Responder {
    info!("Received request: {:?}", req);
    match process_request(&req.into_inner(), &config) {
        Ok(response) => {
            info!("Processed request successfully");
            HttpResponse::Ok().json(response)
        },
        Err(e) => {
            eprint!("Error processing request: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Process taxes for a country
fn process_country_taxes(
    taxes_config: &TaxesConfig,
    country: &str,
    incomes_to_compute: &[f32],
    max_income: f32,
    specific_income: f32,
) -> TaxData {
    let schedule = taxes_config.get_country(&country).unwrap();
    let knot_points = schedule.to_income_amount_schedule(max_income);
    let tax_amounts = match interpolate_segments_parallel(&incomes_to_compute, &knot_points) {
        Ok(value) => value,
        Err(_) => panic!("Error"),
    };
    let tax_amounts_adjusted = exchange_rate_adjustment(&tax_amounts, 1.0); // TODO: Fix up exchange rate computation logic
    let effective_tax_rates =
        compute_effective_tax_rates(&incomes_to_compute, &tax_amounts_adjusted);

    // Get the specific income (neccessary because step size could be too large)
    let specific_tax_amount =
        IncomeTaxSchedule::compute_income_tax(specific_income, &knot_points).unwrap();
    let specific_tax_rate = specific_tax_amount / specific_income;

    TaxData {
        incomes: incomes_to_compute.to_vec(),
        tax_amounts: tax_amounts_adjusted,
        effective_tax_rates: effective_tax_rates,
        specific_tax_amount: Some(specific_tax_amount),
        specific_tax_rate: Some(specific_tax_rate),
    }
}

/// Process breakeven points
fn process_country_breakeven_points(
    country_one: &str,
    country_two: &str,
    taxes_config: &TaxesConfig,
    max_income_to_consider: f32,
) -> BreakevenData {
    // TODO: Neglecting how to handle exchange rates right now but this must be done for breakevens
    let schedule_one = taxes_config.get_country(country_one).unwrap();
    let schedule_two = taxes_config.get_country(country_two).unwrap();
    let breakevens = compute_breakeven_points(
        &schedule_one.to_income_amount_schedule(max_income_to_consider),
        &schedule_two.to_income_amount_schedule(max_income_to_consider),
    );
    let (breakeven_incomes, breakeven_amounts): (Vec<f32>, Vec<f32>) = breakevens
        .iter()
        // The origin is not interesting filter it out
        .filter(|point| !(point.income == 0.0 && point.income_tax_amount == 0.0))
        .map(|point| (point.income, point.income_tax_amount))
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

fn process_request(req: &TaxRequest, taxes_config: &TaxesConfig) -> Result<TaxResponse, String> {
    let step = 10.0; // TODO: Play around with this param
    let min_income = 0.0;
    let incomes_to_compute = generate_range(min_income, req.max_income, step);

    let mut country_specific_data = HashMap::new();

    for country in &req.countries {
        country_specific_data.insert(
            country.clone(),
            process_country_taxes(
                &taxes_config,
                &country,
                &incomes_to_compute,
                req.max_income,
                req.income,
            ),
        );
    }

    let mut country_comb_data = HashMap::new();
    if req.show_break_even {
        for i in 0..req.countries.len() {
            for j in i + 1..req.countries.len() {
                country_comb_data.insert(
                    format!("{}-{}", req.countries[i], req.countries[j]),
                    process_country_breakeven_points(
                        &req.countries[i],
                        &req.countries[j],
                        &taxes_config,
                        req.max_income,
                    ),
                );
            }
        }
    }

    Ok(TaxResponse {
        country_specific_data,
        country_comb_data: if req.show_break_even {
            Some(country_comb_data)
        } else {
            None
        },
    })
}

/// Currency conversion. Exchange rate is in units of local per foriegn.
fn exchange_rate_adjustment(values: &[f32], exchange_rate: f32) -> Vec<f32> {
    values
        .par_iter()
        .map(|&value| value * exchange_rate)
        .collect()
}

// TODO: Some exchange rate stuff should be handled.

#[cfg(test)]
mod tests {
    // Tests go here until they are developed and put into modules.
    // This process will occur as the project matures.
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let taxes_config = TaxesConfig::new("./assets/taxes.json");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(taxes_config.clone()))
            .route("/process", web::post().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
