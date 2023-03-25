use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::io::Error;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(address: &str) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(address)?
        .run();
    Ok(server)
}
