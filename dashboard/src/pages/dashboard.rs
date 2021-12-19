use crate::components::*;
use crate::models::*;
use anyhow::Result;
use sycamore::prelude::*;

#[component(Dashboard<G>)]
pub fn dashboard(props: Result<Vec<AverageKpi>>) -> View<G> {
    match props {
        Ok(averages) => view! {
            render_kpis::RenderKpis(averages)
        },
        Err(_) => view! {
            "Error fetching data."
        },
    }
}
