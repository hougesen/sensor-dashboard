use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
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
        let cors = Cors::default().allow_any_method().allow_any_origin();

        App::new()
            .wrap(cors)
            .data(pool.clone())
            .service(hello)
            .service(post_measurement)
            .service(get_measurements)
            .service(get_measurements_by_location)
            .service(get_measurements_by_type)
            .service(get_measurement_types)
            .service(get_measurement_type_by_id)
            .service(post_measurement_type)
            .service(get_locations)
            .service(get_location_by_id)
            .service(post_location)
            .service(get_average_by_location)
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
