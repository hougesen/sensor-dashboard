use crate::{db, errors::MyError, models::*};
use actix_web::{get, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use std::collections::HashMap;

// Measurement

#[post("/measurement")]
pub async fn post_measurement(
    db_pool: web::Data<Pool>,
    measurement_data: web::Json<Measurement>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement: Measurement = measurement_data.into_inner();
    let new_measurement = db::insert_measurement(&client, measurement).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

#[get("/measurements")]
pub async fn get_measurements(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurements = db::select_measurements(&client).await?;

    let mut response: HashMap<&str, Vec<Measurement>> = HashMap::new();
    response.insert("measurements", measurements);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/measurements/location/{location_id}")]
pub async fn get_measurements_by_location(
    db_pool: web::Data<Pool>,
    location_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let location_id = location_id.into_inner();
    let measurements = db::select_measurements_by_location(&client, location_id).await?;

    let mut response: HashMap<&str, Vec<Measurement>> = HashMap::new();
    response.insert("measurements", measurements);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/measurements/type/{measurement_type_id}")]
pub async fn get_measurements_by_type(
    db_pool: web::Data<Pool>,
    measurement_type_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement_type_id = measurement_type_id.into_inner();
    let measurements = db::select_measurements_by_type(&client, measurement_type_id).await?;

    let mut response: HashMap<&str, Vec<Measurement>> = HashMap::new();
    response.insert("measurements", measurements);

    Ok(HttpResponse::Ok().json(response))
}

// MeasurementType
#[post("/measurement-type")]
pub async fn post_measurement_type(
    db_pool: web::Data<Pool>,
    measurement_type_data: web::Json<MeasurementType>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement_type = measurement_type_data.into_inner();
    let new_measurement = db::insert_measurement_type(&client, measurement_type).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

#[get("/measurement-types")]
pub async fn get_measurement_types(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let measurement_types = db::select_measurement_types(&client).await?;

    let mut response: HashMap<&str, Vec<MeasurementType>> = HashMap::new();
    response.insert("measurement-types", measurement_types);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/measurement-type/{measurement_type_id}")]
pub async fn get_measurement_type_by_id(
    db_pool: web::Data<Pool>,
    measurement_type_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement_type_id = measurement_type_id.into_inner();
    let measurement_type = db::select_measurement_type_by_id(&client, measurement_type_id).await?;

    let mut response: HashMap<&str, MeasurementType> = HashMap::new();
    response.insert("measurement-type", measurement_type);

    Ok(HttpResponse::Ok().json(response))
}

// Location
#[post("/location")]
pub async fn post_location(
    db_pool: web::Data<Pool>,
    location_data: web::Json<Location>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let location = location_data.into_inner();
    let new_location = db::insert_location(&client, location).await?;

    Ok(HttpResponse::Ok().json(new_location))
}

#[get("/locations")]
pub async fn get_locations(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let locations = db::select_locations(&client).await?;
    let mut response: HashMap<&str, Vec<Location>> = HashMap::new();
    response.insert("locations", locations);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/location/{location_id}")]
pub async fn get_location_by_id(
    db_pool: web::Data<Pool>,
    location_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let location_id = location_id.into_inner();
    let location = db::select_location_by_id(&client, location_id).await?;

    let mut response: HashMap<&str, Location> = HashMap::new();
    response.insert("location", location);

    Ok(HttpResponse::Ok().json(response))
}

#[get("/average/{location_id}/{measurement_type_id}")]
pub async fn get_average_by_location(
    db_pool: web::Data<Pool>,
    location_id: web::Path<i32>,
    measurement_type_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let location_id = location_id.into_inner();
    let measurement_type_id = measurement_type_id.into_inner();

    let location = db::get_average_by_location(&client, location_id, measurement_type_id).await?;
    let mut response: HashMap<&str, Vec<AverageKpi>> = HashMap::new();
    response.insert("averages", location);

    Ok(HttpResponse::Ok().json(response))
}
