use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
#[pg_mapper(table = "measurements")]
pub struct Measurement {
    pub measurement_id: Option<i32>,
    pub measurement_type_id: i32,
    pub location_id: i32,
    pub measurement_value: f32,
    pub measurement_time: DateTime<Utc>,
}

#[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
#[pg_mapper(table = "locations")]
pub struct Location {
    pub location_id: Option<i32>,
    pub location_name: String,
}

#[derive(Clone, Deserialize, PostgresMapper, Serialize, Debug, PartialEq)]
#[pg_mapper(table = "measurement_types")]
pub struct MeasurementType {
    pub measurement_type_id: Option<i32>,
    pub measurement_type_name: String,
}
