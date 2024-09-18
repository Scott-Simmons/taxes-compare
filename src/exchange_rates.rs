use rayon::prelude::*;
//use serde::{Deserialize, Serialize};
//use awc::client;
//use std::collections::HashMap;

/// Currency conversion. Exchange rate is in units of local per foriegn.
pub fn exchange_rate_adjustment(values: &[f32], exchange_rate: f32) -> Vec<f32> {
    values
        .par_iter()
        .map(|&value| value * exchange_rate)
        .collect()
}

///// Get exchange rates from endpoint
//async fn fetch_rates(base_currency: str, foriegn_currencies: &[String]) -> HashMap<String, f32> {
//    let endpoint = format!("https://open.er-api.com/v6/latest/{}", base_currency);
//    let mut client = awc::Client::default();
//    let response = client.get(endpoint)
//        .insert_header(("User-Agent", "Actix-web"))
//        .send()
//        .await?;
//}

//#[derive(Debug, Serialize, Deserialize)]
//pub struct ExchangeRatesResponse {
//    result: String,
//    provider: String,
//    documentation: String,
//    terms_of_use: String,
//    time_last_update_unix: u32,
//    time_last_update_utc: String,
//    time_next_update_unix: u32,
//    time_next_update_utc: String,
//    time_eol_unix: u32,
//    base_code: String,
//    rates: ExchangeRates
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//pub struct ExchangeRates {
//    rates: std::collections::HashMap<String, f32>,
//}
