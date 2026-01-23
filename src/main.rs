use crate::handlers::api_handler::{health_check, json_error, not_found};
use actix_web::{App, HttpServer, web};
use std::env;

mod config;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt().with_line_number(true).init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().unwrap_or(8080u16);

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(json_error))
            .route("/health", web::get().to(health_check))
             .default_service(web::route().to(not_found))
    })
        .bind((host, port))?
        .run()
        .await
}
