use actix_web::{web, App, HttpServer};

use taxes_redux::controller::handle_request::handle_request;
use taxes_redux::controller::taxes_config::TaxesConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let taxes_config = TaxesConfig::new("./assets/taxes.json");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(taxes_config.clone()))
            .route("/process", web::post().to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
