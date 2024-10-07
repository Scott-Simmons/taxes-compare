use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;

use taxes_compare::controller::handle_request::handle_request;
use taxes_compare::controller::taxes_config::TaxesConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let taxes_config = TaxesConfig::new(
        &env::var("TAXES_CONFIG_PATH").unwrap_or_else(|_| String::from("./assets/taxes.json")),
    );
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(taxes_config.clone()))
            .route("/process", web::post().to(handle_request))
    })
    .bind(format!(
        "0.0.0.0:{}",
        &env::var("SERVER_PORT").unwrap_or_else(|_| String::from("6000"))
    ))?
    .run()
    .await
}
