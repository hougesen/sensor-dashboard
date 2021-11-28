use crate::{errors::MyError, models::*};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn insert_measurement(
    client: &Client,
    measurement: Measurements,
) -> Result<Measurements, MyError> {
    println!("db insert_measurement {:#?}", measurement);
    let _stmt = include_str!("../queries/insert_measurement.sql");
    let _stmt = _stmt.replace("$table_fields", &Measurements::sql_table_fields());

    println!("_stmt {:?}", _stmt);

    let stmt = client.prepare(&_stmt).await.unwrap();
    println!("done prema");
    let res = client
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
        .ok_or(MyError::NotFound);

    println!("res {:?}", res);
    res
}

pub async fn insert_measurement_type(
    client: &Client,
    measurement_type: MeasurementType,
) -> Result<MeasurementType, MyError> {
    let _stmt = include_str!("../queries/insert_measurement_type.sql");
    let _stmt = _stmt.replace("$table_fields", &MeasurementType::sql_table_fields());
    println!("_stmt {:?}", _stmt);

    let stmt = client.prepare(&_stmt).await.unwrap();

    println!("done prema");
    client
        .query(&stmt, &[&measurement_type.measurement_type_name])
        .await?
        .iter()
        .map(|row| MeasurementType::from_row_ref(row).unwrap())
        .collect::<Vec<MeasurementType>>()
        .pop()
        .ok_or(MyError::NotFound)
}
