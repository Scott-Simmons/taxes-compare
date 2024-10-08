use crate::controller::taxes_config;
use crate::core::points::tax_amount::{IncomeTaxKnot, IncomeTaxPoint};
use crate::core::schedules::amount_schedule::IncomeTaxAmountSchedule;
use crate::core::segment::LinearPiecewiseSegment;
use rayon::prelude::*;

/// Utility function for generating a range of income points.
pub fn generate_range(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let mut values = Vec::new();
    let mut current = start;
    while current <= stop {
        values.push(current);
        current += step;
    }
    values
}

/// Util for adjusting schedule by exchange rate
pub fn adjust_exchange_rate_schedule(
    tax_config: &taxes_config::TaxesConfig,
    country: &str,
    exchange_rate: &Option<f32>,
    max_income_to_consider: f32,
) -> IncomeTaxAmountSchedule {
    tax_config
        .get_country(&country)
        .unwrap()
        .exchange_rate_adjustment(&exchange_rate)
        .to_income_amount_schedule(max_income_to_consider)
}

/// Util for testing that points are approx eq.
/// Used only in testing.
pub fn income_points_are_approx_eq(
    point1: IncomeTaxPoint,
    point2: IncomeTaxPoint,
    tol: f32,
) -> bool {
    let x1 = point1.income();
    let x2 = point2.income();
    let y1 = point1.income_tax_amount();
    let y2 = point2.income_tax_amount();
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() < tol
}

pub fn group_incomes_by_segment(
    incomes: &[f32],
    knot_points: &[IncomeTaxKnot],
) -> Vec<(LinearPiecewiseSegment, Vec<f32>)> {
    // Small bit of inefficiency with segments representation as it doubles up e.g. r1 == l2
    // But this should not matter since number of knot points is usually low.
    // Using LinearPiecewiseSegment makes things more readable too.
    // This can be improved by avoiding cloning.
    let mut point_index = 0;
    let mut incomes_in_segment = Vec::new();
    let mut overall_result = Vec::new();
    let mut income_index = 0;
    while income_index < incomes.len() {
        let income = incomes[income_index];
        if point_index + 1 >= knot_points.len() {
            break;
        }
        // Assumes sorted incomes and sorted knot points.
        if income <= knot_points[point_index + 1].income_limit() {
            incomes_in_segment.push(income);
            income_index += 1;
        } else {
            overall_result.push((
                LinearPiecewiseSegment {
                    left_point: knot_points[point_index].clone(),
                    right_point: knot_points[point_index + 1].clone(),
                },
                incomes_in_segment.clone(),
            ));
            incomes_in_segment.clear();
            point_index += 1;
        }
    }
    // Handle the final segment (wont be flushed otherwise...)
    if !incomes_in_segment.is_empty() && point_index < knot_points.len() - 1 {
        overall_result.push((
            LinearPiecewiseSegment {
                left_point: knot_points[point_index].clone(),
                right_point: knot_points[point_index + 1].clone(),
            },
            incomes_in_segment,
        ));
    }
    overall_result
}

/// Given the tax amounts and the incomes, compute the effective tax rate at each income step.
pub fn compute_effective_tax_rates(incomes: &[f32], income_tax_amounts: &[f32]) -> Vec<f32> {
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

#[cfg(test)]
mod tests {

    use crate::controller::taxes_config;
    use crate::core::points::tax_amount::IncomeTaxKnot;
    use crate::core::schedules::amount_schedule::IncomeTaxAmountSchedule;
    use crate::core::segment::LinearPiecewiseSegment;
    use crate::utils::adjust_exchange_rate_schedule;
    use crate::utils::group_incomes_by_segment;

    #[test]
    fn test_adjust_exchange_rate_schedule() {
        let tax_config = taxes_config::TaxesConfig::new("test_data/foo.json");
        let country = "Foo";
        let max_income_to_consider = 390000.0;
        let schedule_one =
            adjust_exchange_rate_schedule(&tax_config, &country, &None, max_income_to_consider);
        let schedule_two = adjust_exchange_rate_schedule(
            &tax_config,
            &country,
            &Some(2.0),
            max_income_to_consider,
        );
        let schedule_three = adjust_exchange_rate_schedule(
            &tax_config,
            &country,
            &Some(1.0 / 2.0),
            max_income_to_consider,
        );

        assert_eq!(
            schedule_one,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(100000.0, 10000.0),
                IncomeTaxKnot::new(200000.0, 30000.0),
                IncomeTaxKnot::new(300000.0, 60000.0),
                IncomeTaxKnot::new(390000.0, 96000.0)
            ])
        );
        assert_eq!(
            schedule_two,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(50000.0, 5000.0),
                IncomeTaxKnot::new(100000.0, 15000.0),
                IncomeTaxKnot::new(150000.0, 30000.0),
                IncomeTaxKnot::new(390000.0, 126000.0)
            ])
        );
        assert_eq!(
            schedule_three,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(200000.0, 20000.0),
                IncomeTaxKnot::new(390000.0, 58000.0)
            ])
        );

        let max_income_to_consider = 400000.0;
        let schedule_one =
            adjust_exchange_rate_schedule(&tax_config, &country, &None, max_income_to_consider);
        let schedule_two = adjust_exchange_rate_schedule(
            &tax_config,
            &country,
            &Some(2.0),
            max_income_to_consider,
        );
        let schedule_three = adjust_exchange_rate_schedule(
            &tax_config,
            &country,
            &Some(1.0 / 2.0),
            max_income_to_consider,
        );

        assert_eq!(
            schedule_one,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(100000.0, 10000.0),
                IncomeTaxKnot::new(200000.0, 30000.0),
                IncomeTaxKnot::new(300000.0, 60000.0),
                IncomeTaxKnot::new(400000.0, 100000.0)
            ])
        );
        assert_eq!(
            schedule_two,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(50000.0, 5000.0),
                IncomeTaxKnot::new(100000.0, 15000.0),
                IncomeTaxKnot::new(150000.0, 30000.0),
                IncomeTaxKnot::new(400000.0, 130000.0)
            ])
        );
        assert_eq!(
            schedule_three,
            IncomeTaxAmountSchedule::new(vec![
                IncomeTaxKnot::new(0.0, 0.0),
                IncomeTaxKnot::new(200000.0, 20000.0),
                IncomeTaxKnot::new(400000.0, 60000.0)
            ])
        );
    }

    #[test]
    fn test_group_incomes_by_segment() {
        let incomes = vec![500.0, 1500.0, 1700.0, 2500.0, 3500.0];
        // is not upper bounded.
        let knot_points = vec![
            IncomeTaxKnot::new(0.0, 0.0),
            IncomeTaxKnot::new(1000.0, 0.0),
            IncomeTaxKnot::new(2000.0, 1.0),
            IncomeTaxKnot::new(3000.0, 3.0),
        ];

        let expected_result = vec![
            (
                LinearPiecewiseSegment {
                    left_point: IncomeTaxKnot::new(0.0, 0.0),
                    right_point: IncomeTaxKnot::new(1000.0, 0.0),
                },
                vec![500.0],
            ),
            (
                LinearPiecewiseSegment {
                    left_point: IncomeTaxKnot::new(1000.0, 0.0),
                    right_point: IncomeTaxKnot::new(2000.0, 1.0),
                },
                vec![1500.0, 1700.0],
            ),
            (
                LinearPiecewiseSegment {
                    left_point: IncomeTaxKnot::new(2000.0, 1.0),
                    right_point: IncomeTaxKnot::new(3000.0, 3.0),
                },
                vec![2500.0],
            ),
        ];

        let actual_result = group_incomes_by_segment(&incomes, &knot_points);
        assert_eq!(expected_result, actual_result);
    }
}
