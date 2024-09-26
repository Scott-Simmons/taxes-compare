use rayon::prelude::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Hack for now. Currency-Country mapping can be hardcoded here:
pub fn get_currency_country_mapping() -> HashMap<&'static str, &'static str> {
    let mut country_currency_map = HashMap::new();
    country_currency_map.insert("New Zealand", "NZD");
    country_currency_map.insert("Australia", "AUD");
    country_currency_map.insert("United Kingdom", "GBP");
    country_currency_map.insert("Singapore", "SGD");
    country_currency_map.insert("Norway", "NOK");
    country_currency_map.insert("South Africa", "ZAR");
    country_currency_map.insert("Netherlands", "EUR");
    country_currency_map.insert("Ireland", "EUR");
    country_currency_map.insert("Spain", "EUR");
    country_currency_map.insert("United States of America (excl. state taxes)", "USD");
    country_currency_map.insert("Canada (excl. provincial taxes)", "CAD");
    country_currency_map
}

/// Currency conversion. Exchange rate is in units of local per foriegn.
pub fn exchange_rate_adjustment(values: &[f32], exchange_rate: f32) -> Vec<f32> {
    values
        .par_iter()
        .map(|&value| value * exchange_rate)
        .collect()
}

/// Get exchange rates from endpoint
pub async fn fetch_exchange_rates(
    base_currency: &str,
) -> Result<HashMap<String, f32>, Box<dyn Error>> {
    // TODO: More error checking
    let endpoint = format!("https://open.er-api.com/v6/latest/{}", base_currency);
    let resp = reqwest::get(&endpoint).await?.text().await?;
    let rates: ExchangeRatesResponse = serde_json::from_str(&resp).unwrap();
    Ok(rates.rates)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRatesResponse {
    result: String,
    provider: String,
    documentation: String,
    terms_of_use: String,
    time_last_update_unix: u32,
    time_last_update_utc: String,
    time_next_update_unix: u32,
    time_next_update_utc: String,
    time_eol_unix: u32,
    base_code: String,
    pub rates: std::collections::HashMap<String, f32>,
}

#[cfg(test)]
mod tests {
    use crate::exchange_rates::exchange_rate_adjustment;
    //use crate::exchange_rates::fetch_exchange_rates;

    #[test]
    fn test_exchange_rate_adjustment() {
        let values = vec![1.0, 2.0, 0.5, 10.0, 100.0, 1000.0];
        let rate = 0.5;
        let expected_adjustments = vec![0.5, 1.0, 0.25, 5.0, 50.0, 500.0];
        assert_eq!(
            expected_adjustments,
            exchange_rate_adjustment(&values, rate)
        );
    }

    #[tokio::test]
    async fn test_fetch_rates() {
        //print!("{:?}", fetch_exchange_rates("USD").await.unwrap());
        // TODO: Add a test here
    }
}
