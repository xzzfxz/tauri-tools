use std::error::Error;

use actix_web::{App, HttpServer};

mod controller;
mod model;
mod service;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    HttpServer::new(move || App::new().service(controller::search))
        .bind("127.0.0.1:8086")?
        .run()
        .await
}
