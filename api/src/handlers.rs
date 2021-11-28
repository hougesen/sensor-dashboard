use crate::{db, errors::MyError, models::*};
use actix_web::{get, web, App, Error, HttpResponse, HttpServer, Responder};
use deadpool_postgres::{Client, Pool};
use std::collections::HashMap;

// Measurement
pub async fn post_measurement(
    db_pool: web::Data<Pool>,
    measurement_data: web::Json<Measurement>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement: Measurement = measurement_data.into_inner();
    let new_measurement = db::insert_measurement(&client, measurement).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

pub async fn get_measurements(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurements = db::select_measurements(&client).await?;

    let mut response: HashMap<&str, Vec<Measurement>> = HashMap::new();
    response.insert("measurements", measurements);

    Ok(HttpResponse::Ok().json(response))
}

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
pub async fn post_measurement_type(
    db_pool: web::Data<Pool>,
    measurement_type_data: web::Json<MeasurementType>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let measurement_type = measurement_type_data.into_inner();
    let new_measurement = db::insert_measurement_type(&client, measurement_type).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

pub async fn get_measurement_types(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let measurement_types = db::select_measurement_types(&client).await?;

    let mut response: HashMap<&str, Vec<MeasurementType>> = HashMap::new();
    response.insert("measurement-types", measurement_types);

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_measurement_type_by_id() {}

// Location
pub async fn post_location(
    db_pool: web::Data<Pool>,
    location_data: web::Json<Location>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let location = location_data.into_inner();
    let new_location = db::insert_location(&client, location).await?;

    Ok(HttpResponse::Ok().json(new_location))
}

pub async fn get_locations() {}

pub async fn get_location_by_id() {}
