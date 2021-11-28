use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use handlers::*;
use tokio_postgres::NoTls;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Server is starting up");

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(post_measurement)
            .service(get_measurements)
            .service(get_measurements_by_location)
            .service(get_measurements_by_type)
            .service(get_measurement_types)
            .service(post_measurement_type)
            .service(post_location)
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server is running");

    server.await
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("GET /");

    HttpResponse::Ok().body("hello fam squad")
}
