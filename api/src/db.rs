use crate::{errors::MyError, models::*};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

// Measurement
pub async fn insert_measurement(
    client: &Client,
    measurement: Measurement,
) -> Result<Measurement, MyError> {
    let _stmt = "INSERT INTO measurements (measurement_type_id, location_id, measurement_value, measurement_time) VALUES ($1, $2, $3, $4) RETURNING $table_fields;";
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
        .prepare("SELECT * FROM measurements ORDER BY measurement_time DESC;")
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
        .prepare(
            "SELECT * FROM measurements WHERE location_id = $1 ORDER BY measurement_time DESC;",
        )
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
 .prepare("SELECT * FROM measurements WHERE measurement_type_id = $1 ORDER BY measurement_time DESC;")
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
    let _stmt = "INSERT INTO measurement_types (measurement_type_name) VALUES ($1) RETURNING $table_fields;";
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

pub async fn select_measurement_types(client: &Client) -> Result<Vec<MeasurementType>, MyError> {
    let stmt = client
        .prepare("SELECT * FROM measurement_types;")
        .await
        .unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| MeasurementType::from_row_ref(row).unwrap())
        .collect::<Vec<MeasurementType>>();

    Ok(result)
}

pub async fn select_measurement_type_by_id(
    client: &Client,
    measurement_type_id: i32,
) -> Result<MeasurementType, MyError> {
    let stmt = client
        .prepare("SELECT * from measurement_types WHERE measurement_type_id = $1;")
        .await
        .unwrap();

    client
        .query(&stmt, &[&measurement_type_id])
        .await?
        .iter()
        .map(|row| MeasurementType::from_row_ref(row).unwrap())
        .collect::<Vec<MeasurementType>>()
        .pop()
        .ok_or(MyError::NotFound)
}

// Location
pub async fn insert_location(client: &Client, location: Location) -> Result<Location, MyError> {
    let _stmt = "INSERT INTO locations (location_name) VALUES ($1) RETURNING $table_fields;";
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

pub async fn select_locations(client: &Client) -> Result<Vec<Location>, MyError> {
    let stmt = client.prepare("SELECT * FROM locations;").await.unwrap();

    let result = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Location::from_row_ref(row).unwrap())
        .collect::<Vec<Location>>();

    Ok(result)
}

pub async fn select_location_by_id(client: &Client, location_id: i32) -> Result<Location, MyError> {
    let stmt = client
        .prepare("SELECT * FROM locations WHERE location_id = $1;")
        .await
        .unwrap();

    client
        .query(&stmt, &[&location_id])
        .await?
        .iter()
        .map(|row| Location::from_row_ref(row).unwrap())
        .collect::<Vec<Location>>()
        .pop()
        .ok_or(MyError::NotFound)
}
