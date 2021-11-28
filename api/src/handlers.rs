use crate::{db, errors::MyError, models::*};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn insert_measurement(
    db_pool: web::Data<Pool>,
    measurement_data: web::Json<Measurements>,
) -> Result<HttpResponse, Error> {
    let measurement: Measurements = measurement_data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_measurement = db::insert_measurement(&client, measurement).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

pub async fn insert_measurement_type(
    db_pool: web::Data<Pool>,
    measurement_type_data: web::Json<MeasurementType>,
) -> Result<HttpResponse, Error> {
    let measurement_type = measurement_type_data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_measurement = db::insert_measurement_type(&client, measurement_type).await?;

    Ok(HttpResponse::Ok().json(new_measurement))
}

pub async fn insert_location(
    db_pool: web::Data<Pool>,
    location_data: web::Json<Location>,
) -> Result<HttpResponse, Error> {
    let location = location_data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_location = db::insert_location(&client, location).await?;

    Ok(HttpResponse::Ok().json(new_location))
}
