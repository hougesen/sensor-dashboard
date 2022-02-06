use crate::models::*;
use anyhow::Result;
use reqwasm::http::Request;

const API_BASE_URL: &str = "http://localhost:5000";

pub async fn fetch_average_by_location(location_id: i32) -> Result<Vec<AverageKpi>> {
    let url = format!("{}/average/{}", API_BASE_URL, location_id);

    let resp = Request::get(&url).send().await?;

    let body = resp.json::<AverageKpiResponse>().await?;

    Ok(body.averages)
}

pub async fn fetch_measurements_by_type(measurement_type_id: i32) -> Result<Vec<Measurement>> {
    let url = format!("{}/measurements/type/{}", API_BASE_URL, measurement_type_id);

    let resp = Request::get(&url).send().await?;

    let body = resp.json::<MeasurementsTypeResponse>().await?;

    Ok(body.measurements)
}

pub async fn generate_dashboard(location_id: i32) -> Result<DashboardModel> {
    let average_values = fetch_average_by_location(location_id).await;
    let measurements = fetch_measurements_by_type(2).await;

    Ok(DashboardModel {
        averages: average_values.unwrap(),
        measurements: measurements.unwrap(),
    })
}
