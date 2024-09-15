mod errors;
mod models;
mod utils;
use errors::errors::TaxError; // TODO: Organise better
use models::segment::LinearPiecewiseSegment;
use utils::utils::generate_range; // TODO: Organise better

use rayon::prelude::*;
use serde::Deserialize;

// Rest API calls etc. Efficient processing for each country etc...
// Request will come in with (1) Countries list<string>, (2) Income float, (3) Do breakeven points t/f, (4) max_income float, (5) Exchange rate.
// Dispatch off to the things efficiently...

/// A point characterised by a marginal tax rate at a given level of income
#[derive(Clone, Debug, Deserialize)]
struct MarginalRateKnot {
    /// The marginal tax rate f(x) at given income threshold x
    marginal_rate: f32,
    /// The income threshold at which the knot is the boundry point
    income_limit: f32,
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
}

fn main() {
    println!("Hello, world!");
}
