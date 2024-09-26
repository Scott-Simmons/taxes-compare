use crate::controller::taxes_config::{BreakevenData, TaxData, TaxesConfig};
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TaxPlotDataResponse {
    pub country_specific_data: HashMap<String, TaxData>,
    pub country_comb_data: Option<HashMap<String, BreakevenData>>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TaxPlotDataRequest {
    pub countries: Vec<String>,
    pub income: Option<f32>,
    pub max_income: f32,
    pub show_break_even: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizing_currency: Option<String>,
}

pub async fn handle_request(
    req: web::Json<TaxPlotDataRequest>,
    config: web::Data<TaxesConfig>,
) -> impl Responder {
    info!("Received request: {:?}", req);
    let res = match &config.process_request(&req.into_inner()).await {
        Ok(response) => {
            info!("Processed request successfully");
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprint!("Error processing request: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    };
    return res;
}
