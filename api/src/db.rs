use crate::{errors::MyError, models::*};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// Measurement
pub async fn insert_measurement(
    client: &Client,
    measurement: Measurement,
) -> Result<Measurement, MyError> {
    let _stmt = include_str!("../queries/insert_measurement.sql");
    let _stmt = _stmt.replace("$table_fields", &Measurement::sql_table_fields());
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
        .map(|row| Measurement::from_row_ref(row).unwrap())
        .collect::<Vec<Measurement>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn select_measurements(client: &Client) -> Result<Vec<Measurement>, MyError> {
    let stmt = client
        .prepare("SELECT * FROM measurements ORDER BY measurement_time DESC")
        .await
        .unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Measurement::from_row_ref(row).unwrap())
        .collect::<Vec<Measurement>>();

    Ok(result)
}

pub async fn select_measurements_by_location(
    client: &Client,
    location_id: i32,
) -> Result<Vec<Measurement>, MyError> {
    let stmt = client
        .prepare("SELECT * FROM measurements WHERE location_id = $1 ORDER BY measurement_time DESC")
        .await
        .unwrap();

    let result = client
        .query(&stmt, &[&location_id])
        .await?
        .iter()
        .map(|row| Measurement::from_row_ref(row).unwrap())
        .collect::<Vec<Measurement>>();

    Ok(result)
}

pub async fn select_measurements_by_type(
    client: &Client,
    measurement_type_id: i32,
) -> Result<Vec<Measurement>, MyError> {
    let stmt = client
        .prepare("SELECT * FROM measurements WHERE measurement_type_id = $1 ORDER BY measurement_time DESC")
        .await
        .unwrap();

    let result = client
        .query(&stmt, &[&measurement_type_id])
        .await?
        .iter()
        .map(|row| Measurement::from_row_ref(row).unwrap())
        .collect::<Vec<Measurement>>();

    Ok(result)
}

// MeasurementType
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

// Location

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
