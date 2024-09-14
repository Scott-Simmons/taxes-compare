use itertools::Itertools;
use nalgebra;
use std::{cmp::Ordering, collections::HashMap};
// list of (r, b)
// parse this from json for a given country
// get list of (x, get_tax_amount_from_marginal_rates(b, margina_rates_knots))
// to get a single tax amount: get_tax_amount_from_tax_amounts(income, income_tax_knots) (binary
// search to get segment, then linear interpolation)
// To efficiently generate a list of income taxes... linear scan, then linear_interpolation
// income_taxes_list = generate_income_taxes(income_start, income_stop, income_step, income_tax_knots)
// compute_breakeven_points(segments_f1, segments_f2) -> list of breakeven points

#[derive(Debug, PartialEq)]
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
#[derive(Clone, Debug)]
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
    knots1: Vec<IncomeTaxKnot>,
    knots2: Vec<IncomeTaxKnot>,
) -> Vec<IncomeTaxPoint> {
    // Do not forget that for knots derived from tax schedules we need to define an upper bound (cannot be inf, inf...)

    let mut i = 0; // curve 1
    let mut j = 0; // curve 2
    let mut breakeven_points = Vec::new();

    while i + 1 < knots1.len() && j + 1 < knots2.len() {
        let l1 = knots1[i].income_limit;
        let r1 = knots1[i + 1].income_limit;
        let l2 = knots2[j].income_limit;
        let r2 = knots2[j + 1].income_limit;

        let segments_have_overlap = r1 >= l1 && r2 >= l1;

        if segments_have_overlap {
            let candidate_segment = LinearPiecewiseSegment {
                left_point: knots2[j].clone(),
                right_point: knots2[j + 1].clone(),
            };
            let intersection = LinearPiecewiseSegment {
                left_point: knots1[i].clone(),
                right_point: knots1[i + 1].clone(),
            }
            .compute_intersection(&candidate_segment);
            match intersection {
                None => {} // do nothing
                Some(breakeven_point) => {
                    breakeven_points.push(breakeven_point); // add to result set
                }
            };
        }

        if r1 < r2 {
            i += 1; // advance 1
        } else {
            j += 1; // advance 2
        }
    }
    breakeven_points
}

/// A line segment characterised by two points
#[derive(Clone, Debug)]
struct LinearPiecewiseSegment {
    /// Two knot points characterise a segment of a linear piecewise function
    /// https://en.wikipedia.org/wiki/Line_segment
    left_point: IncomeTaxKnot,
    right_point: IncomeTaxKnot,
    // some kind of assert so that left point x value always less than right point x value
}

/// Linearly interpolate a line segment at an income value, to get a taxation value.
impl LinearPiecewiseSegment {
    fn linear_interpolation(&self, income: f32) -> Option<f32> {
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
    /// Gets line segments into a form parameterised as l = a * t(b - a) where a and b are
    /// points in R^2 and t \in [0, 1]
    /// Then equate l1 = l2 and solve under those constraints
    /// The maths yields t1(b1 - a1) + t2(a2 - b2) = (a2 - a1)
    /// This can then be brought into the form At = c
    /// Three types of solution: Independent (one solution), Dependent (infininity many -
    /// return the whole line, inconsistent - return None). Rank(A) = 2, Rank(A) < 2, Rank(A) >
    /// 2
    /// See: https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    /// If the segments are colinear it will return None
    fn compute_intersection(
        &self,
        segment_to_intersect: &LinearPiecewiseSegment,
    ) -> Option<IncomeTaxPoint> {
        let x1 = self.left_point.income_limit;
        let x2 = self.right_point.income_limit;
        let y1 = self.left_point.income_tax_amount;
        let y2 = self.right_point.income_tax_amount;
        let q1 = segment_to_intersect.left_point.income_limit;
        let q2 = segment_to_intersect.right_point.income_limit;
        let r1 = segment_to_intersect.left_point.income_tax_amount;
        let r2 = segment_to_intersect.right_point.income_tax_amount;
        //
        // https://docs.rs/nalgebra/latest/nalgebra/macro.matrix.html
        let matrix_lhs = nalgebra::matrix![
            x2 - x1, q1 - q2;
            y2 - y1, r1 - r2
        ];
        let row_size: usize = 2;
        let tol: f32 = 0.00000001;
        match matrix_lhs.rank(tol).cmp(&row_size) {
            Ordering::Less => None,    // linearly dependent - coincident
            Ordering::Greater => None, // overdetermined, will never happen
            Ordering::Equal => {
                // solve Ax=b
                let b_rhs = nalgebra::matrix![q1 - x1; r1 - y1];
                match matrix_lhs.lu().solve(&b_rhs) {
                    None => None,
                    Some(solution) => {
                        // Verify solution satisfies t \in [0, 1]
                        let (t1, t2) = (solution[(0, 0)], solution[(1, 0)]);
                        if (0.0 <= t1 && t1 <= 1.0) && (0.0 <= t2 && t2 <= 1.0) {
                            Some(IncomeTaxPoint {
                                income_tax_amount: y1 + t1 * (y2 - y1),
                                income: x1 + t1 * (x2 - x1),
                            })
                        } else {
                            None // No intersection within the segment range.
                        }
                    }
                }
            }
        }
    }
}

/// TODO: This might not be needed becuase the curve can be parameterised by the income knots.
/// Will still be needed
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
    use crate::{
        IncomeTaxKnot, IncomeTaxPoint, IncomeTaxSchedule, LinearPiecewiseSegment, MarginalRateKnot,
        TaxError,
    };

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
                MarginalRateKnot {
                    marginal_rate: 0.1,
                    income_limit: 10000.0,
                },
                MarginalRateKnot {
                    marginal_rate: 0.2,
                    income_limit: 20000.0,
                },
                MarginalRateKnot {
                    marginal_rate: 0.3,
                    income_limit: f32::INFINITY,
                },
            ],
        };

        let result = schedule.get_tax_amount_from_marginal_rates_knots(25000.0);
        assert_eq!(result.unwrap(), 4500.0);

        let invalid_result = schedule.get_tax_amount_from_marginal_rates_knots(-25000.0);
        assert_eq!(invalid_result.unwrap_err(), TaxError::NegativeIncome);

        let zero_result = schedule.get_tax_amount_from_marginal_rates_knots(0.0);
        assert_eq!(zero_result.unwrap(), 0.0);
    }

    #[test]
    fn test_get_breakeven_point() {
        // https://www.desmos.com/calculato
        let test_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 10.0,
                income_tax_amount: 0.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 10.0,
            },
        };
        let interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 10.0,
                income_tax_amount: 10.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };

        let result = test_segment.compute_intersection(&interecting_segment);
        assert_eq!(
            result.unwrap(),
            IncomeTaxPoint {
                income_tax_amount: 5.0,
                income: 5.0
            }
        );

        let barely_interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 5.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };
        let result = test_segment.compute_intersection(&barely_interecting_segment);
        assert_eq!(
            result.unwrap(),
            IncomeTaxPoint {
                income_tax_amount: 5.0,
                income: 5.0
            }
        );

        let non_interecting_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 4.0,
                income_tax_amount: 4.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
        };
        let result = test_segment.compute_intersection(&non_interecting_segment);
        assert!(result.is_none());

        let result = test_segment.compute_intersection(&test_segment);
        assert!(result.is_none());

        let parallel_segment = LinearPiecewiseSegment {
            left_point: IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 5.0,
            },
            right_point: IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 0.0,
            },
        };
        let result = parallel_segment.compute_intersection(&test_segment);
        assert!(result.is_none());
    }

    fn income_points_are_approx_eq(
        point1: IncomeTaxPoint,
        point2: IncomeTaxPoint,
        tol: f32,
    ) -> bool {
        let x1 = point1.income;
        let x2 = point2.income;
        let y1 = point1.income_tax_amount;
        let y2 = point2.income_tax_amount;
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() < tol
    }

    #[test]
    fn test_get_breakeven_points() {
        use crate::compute_breakeven_points;
        use assert_approx_eq::assert_approx_eq;

        // https://www.desmos.com/calculator
        // Breakevens at (25/3, 20/3) and (50/3, 40/3)

        let curve1 = vec![
            IncomeTaxKnot {
                income_limit: 0.0,
                income_tax_amount: 0.0,
            },
            IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 5.0,
            },
            IncomeTaxKnot {
                income_limit: 15.0,
                income_tax_amount: 10.0,
            },
            IncomeTaxKnot {
                income_limit: 20.0,
                income_tax_amount: 20.0,
            },
        ];
        let curve2 = vec![
            IncomeTaxKnot {
                income_limit: 5.0,
                income_tax_amount: 0.0,
            },
            IncomeTaxKnot {
                income_limit: 10.0,
                income_tax_amount: 10.0,
            },
            IncomeTaxKnot {
                income_limit: 20.0,
                income_tax_amount: 15.0,
            },
        ];

        let breakevens = compute_breakeven_points(curve1, curve2);
        let tolerance = 1e-5;
        assert!(income_points_are_approx_eq(
            breakevens[0].clone(),
            IncomeTaxPoint {
                income: 25.0 / 3.0,
                income_tax_amount: 20.0 / 3.0
            },
            tolerance
        ));

        assert!(income_points_are_approx_eq(
            breakevens[1].clone(),
            IncomeTaxPoint {
                income: 50.0 / 3.0,
                income_tax_amount: 40.0 / 3.0
            },
            tolerance
        ));

        print!("{:?}", breakevens)
    }
}

fn main() {
    println!("Hello, world!");
}
