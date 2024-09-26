use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

/// A point characterised by a marginal tax rate at a given level of income
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarginalRateKnot {
    /// The marginal tax rate f(x) at given income threshold x
    marginal_rate: f32,
    /// The income threshold at which the knot is the boundry point
    #[serde(deserialize_with = "null_to_infinity")]
    income_limit: Option<f32>, // unbounded at the last entry
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

impl MarginalRateKnot {
    /// Example: IncomeTaxKnot::new(x,y)
    pub fn new(income_limit: f32, marginal_rate: f32) -> Self {
        Self {
            income_limit: Some(income_limit),
            marginal_rate,
        }
    }

    pub fn marginal_rate(&self) -> f32 {
        self.marginal_rate
    }
    pub fn income_limit(&self) -> Option<f32> {
        self.income_limit
    }
}
