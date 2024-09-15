use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TaxData {
    pub incomes: Vec<f32>,
    pub tax_amounts: Vec<f32>,
    pub effective_tax_rates: Vec<f32>,
    pub specific_tax_amount: Option<f32>,
    pub specific_tax_rate: Option<f32>,
}

#[derive(Serialize)]
pub struct BreakevenData {
    pub breakeven_incomes: Vec<f32>,
    pub breakeven_tax_amounts: Vec<f32>,
    pub breakeven_effective_tax_rates: Vec<f32>,
}

#[derive(Serialize)]
pub struct TaxResponse {
    pub country_specific_data: HashMap<String, TaxData>,
    pub country_comb_data: Option<HashMap<String, BreakevenData>>,
}
