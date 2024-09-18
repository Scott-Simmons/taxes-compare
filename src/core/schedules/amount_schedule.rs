use crate::core::points::tax_amount::{IncomeTaxKnot, IncomeTaxPoint};
use crate::core::segment::LinearPiecewiseSegment;
use crate::errors::TaxError;
use crate::utils::{generate_range, group_incomes_by_segment};
use rayon::prelude::*;
use serde::Deserialize;

/// Schedule representing how tax amounts change at each income threshold.
#[derive(Deserialize, Debug, Clone)]
pub struct IncomeTaxAmountSchedule {
    /// A sorted vector of points where the slope of the tax amount changes.
    schedule: Vec<IncomeTaxKnot>,
}

impl IncomeTaxAmountSchedule {
    pub fn new(income_tax_knots: Vec<IncomeTaxKnot>) -> Self {
        Self {
            schedule: income_tax_knots,
        }
    }

    pub fn schedule(&self) -> &Vec<IncomeTaxKnot> {
        &self.schedule
    }

    /// Compute income tax amounts for a range of incomes
    pub fn compute_income_taxes_in_range(
        &self,
        income_start: f32,
        income_stop: f32,
        income_step: f32,
    ) -> Result<Vec<f32>, TaxError> {
        // Not tested yet (its been tested with the endpoint, but not a formal software test)
        let incomes_to_compute = generate_range(income_start, income_stop, income_step);
        self.compute_income_taxes(&incomes_to_compute)
    }

    /// Breakeven taxes for two schedules.
    /// e.g. incomes where tax amounts of
    /// two countries are equal.
    /// Only compute intersections on
    /// overlapping segments.
    pub fn compute_breakeven_taxes(&self, other_schedule: &Self) -> Vec<IncomeTaxPoint> {
        // Do not forget that for knots derived from tax schedules we need to define an upper bound (cannot be inf, inf...)

        let mut i = 0; // curve 1
        let mut j = 0; // curve 2
        let mut breakeven_points = Vec::new();

        while i + 1 < self.schedule.len() && j + 1 < other_schedule.schedule.len() {
            let l1 = &self.schedule[i].income_limit();
            let r1 = &self.schedule[i + 1].income_limit();
            let l2 = &other_schedule.schedule[j].income_limit();
            let r2 = &other_schedule.schedule[j + 1].income_limit();

            let segments_have_overlap = r1 >= l2 && r2 >= l1;

            if segments_have_overlap {
                let candidate_segment = LinearPiecewiseSegment {
                    left_point: other_schedule.schedule[j].clone(),
                    right_point: other_schedule.schedule[j + 1].clone(),
                };
                let intersection = LinearPiecewiseSegment {
                    left_point: self.schedule[i].clone(),
                    right_point: self.schedule[i + 1].clone(),
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

    /// Given income tax knots and a range of incomes, group points into
    /// their respective linear segments and then interpolate within the
    /// appropriate segment.
    pub fn compute_income_taxes(&self, incomes: &[f32]) -> Result<Vec<f32>, TaxError> {
        if incomes.last().unwrap() > &self.schedule.last().unwrap().income_limit() {
            return Err(TaxError::IncomeOutOfBounds);
        }
        // Choose not to parallelise the segments because the number of segments are usually low.
        let grouped_income_values = group_incomes_by_segment(&incomes, &self.schedule);
        Ok(grouped_income_values
            .par_iter()
            .flat_map(|(segment, income_group)| {
                income_group
                    .par_iter()
                    .map(|&income| segment.linear_interpolation(income).unwrap())
            })
            .collect())
    }

    /// Given income tax knots, do binary search to find the appropriate linear segment
    /// and then interpolate at some level of income in the segment.
    pub fn compute_specific_income_tax(&self, income: f32) -> Option<f32> {
        // TODO: This doesn't catch all the edge cases but should be good enough for now.
        if income < 0.0 {
            return None;
        }
        let mut l = 0;
        let mut r = self.schedule.len();
        while l < r {
            // Cut down search space
            let mid = l + (r - l) / 2;
            if self.schedule[mid].income_limit() == income {
                return Some(self.schedule[mid].income_tax_amount());
            }
            if self.schedule[mid].income_limit() < income {
                if self.schedule[l].income_limit() <= income
                    && income <= self.schedule[l + 1].income_limit()
                {
                    return LinearPiecewiseSegment {
                        left_point: IncomeTaxKnot::new(
                            self.schedule[l].income_limit(),
                            self.schedule[l].income_tax_amount(),
                        ),
                        right_point: IncomeTaxKnot::new(
                            self.schedule[l + 1].income_limit(),
                            self.schedule[l + 1].income_tax_amount(),
                        ),
                    }
                    .linear_interpolation(income);
                }
                l = mid;
            } else {
                if self.schedule[r - 1].income_limit() <= income
                    && income <= self.schedule[r].income_limit()
                {
                    return LinearPiecewiseSegment {
                        left_point: IncomeTaxKnot::new(
                            self.schedule[r - 1].income_limit(),
                            self.schedule[r - 1].income_tax_amount(),
                        ),
                        right_point: IncomeTaxKnot::new(
                            self.schedule[r].income_limit(),
                            self.schedule[r].income_tax_amount(),
                        ),
                    }
                    .linear_interpolation(income);
                }
                r = mid;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::core::points::tax_amount::{IncomeTaxKnot, IncomeTaxPoint};
    use crate::core::schedules::amount_schedule::IncomeTaxAmountSchedule;
    use crate::errors::TaxError;
    use crate::utils::income_points_are_approx_eq;
    #[test]
    fn test_compute_income_taxes() {
        let incomes = vec![500.0, 1500.0, 1700.0, 2500.0];
        // not upper bounded.
        let knot_points = vec![
            IncomeTaxKnot::new(0.0, 0.0), // always need a (0,0) for it to work.
            IncomeTaxKnot::new(1000.0, 0.0),
            IncomeTaxKnot::new(2000.0, 1.0),
            IncomeTaxKnot::new(3000.0, 3.0),
        ];
        let schedule = IncomeTaxAmountSchedule::new(knot_points);
        let actual_result = schedule.compute_income_taxes(&incomes);
        let expected_result = vec![
            0.0, 0.5, 0.7, // y = (1/1000)*(x - 2000) + 1
            2.0,
        ];
        assert_eq!(actual_result, Ok(expected_result));
        // it should throw error because income is out of bounds w.r.t. the tax schedule
        let invalid_incomes = vec![500.0, 1500.0, 1700.0, 2500.0, 3500.0];
        let invalid_result = schedule.compute_income_taxes(&invalid_incomes);
        assert_eq!(invalid_result.unwrap_err(), TaxError::IncomeOutOfBounds);
    }
    #[test]
    fn test_get_tax_amounts_from_tax_amounts_schedule() {
        // Using example from wikipedia: https://en.wikipedia.org/wiki/Progressive_tax
        let income_tax_knots = vec![
            IncomeTaxKnot::new(0.0, 0.0),
            IncomeTaxKnot::new(10000.0, 1000.0),
            IncomeTaxKnot::new(20000.0, 3000.0),
            // An explicit upper bound must be defined in this format.
            // This is known as the "max income to consider"
            IncomeTaxKnot::new(100000.0, 27000.0),
        ];
        let schedule = IncomeTaxAmountSchedule::new(income_tax_knots);
        let result = schedule.compute_specific_income_tax(25000.0);
        assert_eq!(result, Some(4500.0));
        let result = schedule.compute_specific_income_tax(5000.0);
        assert_eq!(result, Some(500.0));
        let invalid_result = schedule.compute_specific_income_tax(-25000.0);
        assert!(invalid_result.is_none());
        let zero_result = schedule.compute_specific_income_tax(0.0);
        assert_eq!(zero_result, Some(0.0));
    }

    #[test]
    fn test_get_breakeven_taxes() {
        // https://www.desmos.com/calculator
        // Breakevens at (25/3, 20/3) and (50/3, 40/3)
        let knots1 = vec![
            IncomeTaxKnot::new(0.0, 0.0),
            IncomeTaxKnot::new(5.0, 5.0),
            IncomeTaxKnot::new(15.0, 10.0),
            IncomeTaxKnot::new(20.0, 20.0),
        ];
        let knots2 = vec![
            IncomeTaxKnot::new(5.0, 0.0),
            IncomeTaxKnot::new(10.0, 10.0),
            IncomeTaxKnot::new(20.0, 15.0),
        ];
        let schedule1 = IncomeTaxAmountSchedule::new(knots1);
        let schedule2 = IncomeTaxAmountSchedule::new(knots2);
        let breakevens = schedule1.compute_breakeven_taxes(&schedule2);
        let tolerance = 1e-5;
        assert!(income_points_are_approx_eq(
            breakevens[0].clone(),
            IncomeTaxPoint::new(25.0 / 3.0, 20.0 / 3.0),
            tolerance
        ));
        assert!(income_points_are_approx_eq(
            breakevens[1].clone(),
            IncomeTaxPoint::new(50.0 / 3.0, 40.0 / 3.0),
            tolerance
        ));
    }
}
