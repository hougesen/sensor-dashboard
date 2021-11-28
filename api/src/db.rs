use crate::{errors::MyError, models::*};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn insert_measurement(
    client: &Client,
    measurement: Measurements,
) -> Result<Measurements, MyError> {
    let _stmt = include_str!("../queries/insert_measurement.sql");
    let _stmt = _stmt.replace("$table_fields", &Measurements::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &measurement.measurement_type_id,
                &measurement.location_id,
                &measurement.measurement_value,
                &measurement.measurement_time,
            ],
        )
        .await?
        .iter()
        .map(|row| Measurements::from_row_ref(row).unwrap())
        .collect::<Vec<Measurements>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn insert_measurement_type(
    client: &Client,
    measurement_type: MeasurementType,
) -> Result<MeasurementType, MyError> {
    let _stmt = include_str!("../queries/insert_measurement_type.sql");
    let _stmt = _stmt.replace("$table_fields", &MeasurementType::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&measurement_type.measurement_type_name])
        .await?
        .iter()
        .map(|row| MeasurementType::from_row_ref(row).unwrap())
        .collect::<Vec<MeasurementType>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn insert_location(client: &Client, location: Location) -> Result<Location, MyError> {
    let _stmt = include_str!("../queries/insert_location.sql");
    let _stmt = _stmt.replace("$table_fields", &Location::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&location.location_name])
        .await?
        .iter()
        .map(|row| Location::from_row_ref(row).unwrap())
        .collect::<Vec<Location>>()
        .pop()
        .ok_or(MyError::NotFound)
}
