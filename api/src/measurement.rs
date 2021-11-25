use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Locations {
    location_id: u32,
    location_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MeasurementTypes {
    measurement_type_id: u32,
    measurement_type_name: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Measurements {
    pub measurement_id: Option<u32>,
    pub measurement_type_id: u32,
    pub location_id: u32,
    pub measurement_value: f32,
    pub measurement_time: String,
}

#[post("/measurement/new")]
pub async fn insert_measurement(measurement: web::Json<Measurements>) -> impl Responder {
    println!("POST /measurement");

    println!("{:?}", measurement);

    HttpResponse::Ok().json(measurement)
}

#[get("/measurements/all")]
pub async fn fetch_measurements_all() -> HttpResponse {
    HttpResponse::Ok().json("asd")
}

#[get("/measurements/location/{location_id}")]
pub async fn fetch_measurements_by_location(location: web::Path<String>) -> HttpResponse {
    let location = location.into_inner();

    println!("GET /measurements/location/{location}", location = location);

    HttpResponse::Ok().json(location)
}
