use crate::core::points::marginal_rate_knot::MarginalRateKnot;
use crate::core::points::tax_amount::IncomeTaxKnot;
use crate::errors::TaxError;
use serde::Deserialize;

use super::amount_schedule::IncomeTaxAmountSchedule;

/// A schedule characterised by changes in marginal rates.
#[derive(Deserialize, Debug, Clone)]
pub struct MarginalIncomeTaxRateSchedule {
    /// A sorted vector of points where the marginal tax rates change.
    schedule: Vec<MarginalRateKnot>,
}

/// An income tax table in terms of marginal rates and income thresholds
impl MarginalIncomeTaxRateSchedule {
    pub fn schedule(&self) -> &Vec<MarginalRateKnot> {
        &self.schedule
    }

    pub fn new(marginal_rate_knots: Vec<MarginalRateKnot>) -> Self {
        Self {
            schedule: marginal_rate_knots,
        }
    }

    /// Get tax amount from the marginal rates schedule.
    /// Dot((r_i - r_{i-1}), max(0, x - b_{i-1}) where (b_0, r_0) = (0,0)
    fn get_tax_amount_from_marginal_rates_knots(&self, income: f32) -> Result<f32, TaxError> {
        if income < 0.0 {
            return Err(TaxError::NegativeIncome(income));
        }
        let marginal_tax_rates_knots = &self.schedule;
        let mut tax_amount = 0.0;
        for (i, marginal_tax_knot) in marginal_tax_rates_knots.iter().enumerate() {
            let prev_limit = if i > 0 {
                marginal_tax_rates_knots[i - 1].income_limit()
            } else {
                Some(0.0)
            };
            let prev_rate = if i > 0 {
                marginal_tax_rates_knots[i - 1].marginal_rate()
            } else {
                0.0
            };
            tax_amount += (marginal_tax_knot.marginal_rate() - prev_rate)
                * (income - prev_limit.expect("Error")).max(0.0);
        }
        Ok(tax_amount)
    }

    /// Adjust the marginal amount schedule according to an exchange rate
    pub fn exchange_rate_adjustment(&self, exchange_rate: &Option<f32>) -> Self {
        match exchange_rate {
            Some(rate) => MarginalIncomeTaxRateSchedule::new(
                self.schedule
                    .clone()
                    .into_iter()
                    .map(|knot| {
                        MarginalRateKnot::new(
                            knot.income_limit().map(|income| income * (1.0 / rate)),
                            knot.marginal_rate(),
                        )
                    })
                    .collect(),
            ),
            None => self.clone(),
        }
    }

    /// Convert to a representation that is better for efficient computation of taxes
    pub fn to_income_amount_schedule(
        &self,
        max_income_to_consider: f32,
    ) -> IncomeTaxAmountSchedule {
        let mut income_tax_knots = vec![IncomeTaxKnot::new(0.0, 0.0)];
        for (i, marginal_rate_knot) in self.schedule.iter().enumerate() {
            if i == self.schedule.len() - 1
                || marginal_rate_knot.income_limit().unwrap() >= max_income_to_consider
            {
                break;
            }
            income_tax_knots.push(IncomeTaxKnot::new(
                marginal_rate_knot.income_limit().expect("Error"),
                self.get_tax_amount_from_marginal_rates_knots(
                    marginal_rate_knot.income_limit().expect("Error"),
                )
                .expect("Error"),
            ));
        }
        income_tax_knots.push(IncomeTaxKnot::new(
            max_income_to_consider,
            self.get_tax_amount_from_marginal_rates_knots(max_income_to_consider)
                .expect("Error"),
        ));
        IncomeTaxAmountSchedule::new(income_tax_knots)
    }
}

#[cfg(test)]
mod tests {

    use crate::core::points::marginal_rate_knot::MarginalRateKnot;
    use crate::core::points::tax_amount::IncomeTaxKnot;
    use crate::core::schedules::marginal_schedule::MarginalIncomeTaxRateSchedule;
    use crate::errors::TaxError;

    #[test]
    fn test_marginal_rates_schedule_to_income_tax_amount_schedule() {
        let schedule = MarginalIncomeTaxRateSchedule {
            schedule: vec![
                MarginalRateKnot::new(Some(10000.0), 0.1),
                MarginalRateKnot::new(Some(20000.0), 0.2),
                MarginalRateKnot::new(Some(f32::INFINITY), 0.3),
            ],
        };
        let max_income_to_consider = 100000.0;
        let expected_result = vec![
            IncomeTaxKnot::new(0.0, 0.0),
            IncomeTaxKnot::new(10000.0, 1000.0),
            IncomeTaxKnot::new(20000.0, 3000.0),
            IncomeTaxKnot::new(max_income_to_consider, 27000.0),
        ];
        assert_eq!(
            &expected_result,
            schedule
                .to_income_amount_schedule(max_income_to_consider)
                .schedule()
        );
    }

    #[test]
    fn test_get_tax_amounts_from_marginal_tax_rates_schedule() {
        // Using example from wikipedia: https://en.wikipedia.org/wiki/Progressive_tax
        let schedule = MarginalIncomeTaxRateSchedule {
            schedule: vec![
                MarginalRateKnot::new(Some(10000.0), 0.1),
                MarginalRateKnot::new(Some(20000.0), 0.2),
                MarginalRateKnot::new(Some(f32::INFINITY), 0.3),
            ],
        };
        let result = schedule.get_tax_amount_from_marginal_rates_knots(25000.0);
        assert_eq!(result.unwrap(), 4500.0);
        let invalid_result = schedule.get_tax_amount_from_marginal_rates_knots(-25000.0);
        assert_eq!(
            invalid_result.unwrap_err(),
            TaxError::NegativeIncome(-25000.0)
        );
        let zero_result = schedule.get_tax_amount_from_marginal_rates_knots(0.0);
        assert_eq!(zero_result.unwrap(), 0.0);
    }
}
