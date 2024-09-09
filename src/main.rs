use log::debug;
use std::collections::HashMap;
// list of (r, b)
// parse this from json for a given country
// get_tax_amount_from_marginal_rates(income, marginal_rates_knots)
// get list of (x, get_tax_amount_from_marginal_rates(b, margina_rates_knots))
// to get a single tax amount: get_tax_amount_from_tax_amounts(income, income_tax_knots) (binary
// search to get segment, then linear interpolation)
// linear_interpolation(segment1, evaluated_at_x) to get a value y
// To efficiently generate a list of income taxes... linear scan, then linear_interpolation
// income_taxes_list = generate_income_taxes(income_start, income_stop, income_step, income_tax_knots)
// compute_breakeven_points(segments_f1, segments_f2) -> list of breakeven points

#[derive(Debug)]
#[derive(PartialEq)]
enum TaxError {
    NegativeIncome,
}

/// A point characterised by a marginal tax rate at a given level of income
struct MarginalRateKnot {
    /// The marginal tax rate f(x) at given income threshold x
    marginal_rate: f32,
    /// The income threshold at which the knot is the boundry point
    income_limit: f32,
}

/// A point characterised by tax amount at given income, which is also denoted as a knot point
struct IncomeTaxKnot {
    /// Income tax amount f(x) for a given maximimum income level x
    income_tax_amount: f32,
    /// The income threshold at which the knot acts as the boundry point
    income_limit: f32,
}

/// A point characterised by tax amount at a given income
struct IncomeTaxPoint {
    /// Income tax amount f(x) for given level of income x
    income_tax_amount: f32,
    /// Level of income x
    income: f32,
}

/// A taxes config represents all information available.
struct TaxesConfig {
    /// Mapping from country to its tax schedule.
    country_map: HashMap<String, IncomeTaxSchedule>,
}
impl TaxesConfig {
    fn new(config_path: &str) -> TaxesConfig {
        TaxesConfig {
            country_map: HashMap::new(),
        }
    }
    fn get_country(&self, country: &str) -> Option<&IncomeTaxSchedule> {
        self.country_map.get(country)
    }
}

/// An income tax table is represented here
struct IncomeTaxSchedule {
    /// A sorted vector of points where the marginal tax rates change.
    schedule: Vec<MarginalRateKnot>,
}
impl IncomeTaxSchedule {
    fn get_tax_amount_from_marginal_rates_knots(&self, income: f32) -> Result<f32, TaxError> {
        if income < 0.0 {
        return Err(TaxError::NegativeIncome);
        }
        let marginal_tax_rates_knots = &self.schedule;
        let mut tax_amount = 0.0;
        for (i, marginal_tax_knot) in marginal_tax_rates_knots.iter().enumerate() {
            let prev_limit = if i > 0 {
                marginal_tax_rates_knots[i - 1].income_limit
            } else {
                0.0
            };
            let prev_rate = if i > 0 {
                marginal_tax_rates_knots[i - 1].marginal_rate
            } else {
                0.0
            };
            tax_amount +=
                (marginal_tax_knot.marginal_rate - prev_rate) * (income - prev_limit).max(0.0);
        }
        Ok(tax_amount)
    }
}

fn compute_breakeven_points(
    segment_1: Vec<LinearPiecewiseSegment>,
    segment_2: Vec<LinearPiecewiseSegment>,
) -> IncomeTaxPoint {
    IncomeTaxPoint {
        income_tax_amount: 0.0,
        income: 0.0,
    }
}

/// A line segment characterised by two points
struct LinearPiecewiseSegment {
    /// Two knot points characterise a segment of a linear piecewise function
    left_point: IncomeTaxKnot,
    right_point: IncomeTaxKnot,
}
impl LinearPiecewiseSegment {
    fn linear_interpolation(&self, income: f32) -> Option<f32> {
        debug!("Income: {}", income);
        println!("Income: {}", income);

        if income < f32::min(self.left_point.income_limit, self.right_point.income_limit)
            || income > f32::max(self.right_point.income_limit, self.left_point.income_limit)
        {
            return None;
        }
        Some(
            self.left_point.income_tax_amount
                + ((self.right_point.income_tax_amount - self.left_point.income_tax_amount)
                    * (income - self.left_point.income_limit))
                    / (self.right_point.income_limit - self.left_point.income_limit),
        )
    }
    // See: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    fn compute_intersection(
        &self,
        segment_to_intersect: LinearPiecewiseSegment,
    ) -> Option<IncomeTaxPoint> {
        None
    }
}

/// TODO: This might not be needed becuase the curve can be parameterised by the income knots.
fn compute_income_taxes(
    income_start: f32,
    income_stop: f32,
    income_step: f32,
    income_tax_knots: Vec<IncomeTaxKnot>,
) -> Vec<IncomeTaxPoint> {
    Vec::new()
}

// TODO: Some exchange rate stuff should be handled.

// Main driver functions

// TODO: Sig should change
fn get_income_taxes(
    country: String,
    taxes_config: &TaxesConfig,
    income_start: f32,
    income_stop: f32,
    income_step: f32,
) -> Vec<IncomeTaxPoint> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::{IncomeTaxKnot, IncomeTaxSchedule, LinearPiecewiseSegment, MarginalRateKnot, TaxError};

    #[test]
    fn test_linear_interpolation() {
        let segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 3.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 4.0,
                income_tax_amount: 6.0,
            },
        };
        let valid_result = segment.linear_interpolation(4.5);
        assert_eq!(valid_result, Some(4.5));

        let invalid_result = segment.linear_interpolation(5.1);
        assert_eq!(invalid_result, None);

        let invalid_result_2 = segment.linear_interpolation(3.9);
        assert_eq!(invalid_result_2, None);
    }

    #[test]
    fn test_get_tax_amounts_from_marginal_tax_rates_schedule() {
        // Using example from wikipedia: https://en.wikipedia.org/wiki/Progressive_tax
        let schedule = IncomeTaxSchedule {
            schedule: vec![
                MarginalRateKnot {marginal_rate: 0.1, income_limit: 10000.0},
                MarginalRateKnot {marginal_rate: 0.2, income_limit: 20000.0},
                MarginalRateKnot {marginal_rate: 0.3, income_limit: f32::INFINITY},
            ]
        };
        
        let result = schedule.get_tax_amount_from_marginal_rates_knots(25000.0);
        assert_eq!(result.unwrap(), 4500.0);

        let invalid_result = schedule.get_tax_amount_from_marginal_rates_knots(-25000.0);
        assert_eq!(invalid_result.unwrap_err(), TaxError::NegativeIncome);

        let zero_result = schedule.get_tax_amount_from_marginal_rates_knots(0.0);
        assert_eq!(zero_result.unwrap(), 0.0);

    }
}

fn main() {
    println!("Hello, world!");
}
