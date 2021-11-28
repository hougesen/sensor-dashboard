use crate::{db, errors::MyError, models::*};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

pub async fn insert_measurement(
    db_pool: web::Data<Pool>,
    measurement_data: web::Json<Measurements>,
) -> Result<HttpResponse, Error> {
    println!("insert_measurement handler");

    let measurement: Measurements = measurement_data.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_measurement = db::insert_measurement(&client, measurement).await?;
    println!("done inserting");
    println!("new_measurement {:?}", new_measurement);

    Ok(HttpResponse::Ok().json(new_measurement))
}

pub async fn insert_measurement_type(
    db_pool: web::Data<Pool>,
    measurement_type_data: web::Json<MeasurementType>,
) -> Result<HttpResponse, Error> {
    println!("insert_measurement_type handler");
    let measurement_type = measurement_type_data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_measurement = db::insert_measurement_type(&client, measurement_type).await?;
    println!("done inserting");
    println!("new_measurement {:?}", new_measurement);

    Ok(HttpResponse::Ok().json(new_measurement))
}
