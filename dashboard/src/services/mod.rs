use crate::models::*;
use anyhow::Result;
use reqwasm::http::Request;

const API_BASE_URL: &str = "http://192.168.87.64:5000";

pub async fn fetch_average_by_location(location_id: i32) -> Result<Vec<AverageKpi>> {
    let url = format!("{}/average/{}", API_BASE_URL, location_id);

    let resp = Request::get(&url).send().await?;

    let body = resp.json::<AverageKpiResponse>().await?;

    Ok(body.averages)
}
