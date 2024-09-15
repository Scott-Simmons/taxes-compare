use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TaxRequest {
    pub countries: Vec<String>,
    pub income: f32,
    pub max_income: f32,
    pub show_break_even: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizing_currency: Option<String>,
}
