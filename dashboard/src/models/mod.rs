use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct Measurement {
    pub measurement_id: Option<i32>,
    pub measurement_type_id: i32,
    pub location_id: i32,
    pub measurement_value: f32,
    pub measurement_time: DateTime<Utc>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct AverageKpiResponse {
    pub averages: Vec<AverageKpi>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct AverageKpi {
    pub measurement_name: String,
    pub location_name: String,
    pub average_value: f64,
}
