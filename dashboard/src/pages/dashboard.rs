use crate::components::*;
use crate::models::*;
use anyhow::Result;
use sycamore::prelude::*;

#[component(Dashboard<G>)]
pub fn dashboard(props: Result<DashboardModel>) -> View<G> {
    match props {
        Ok(props) => view! {
            render_kpis::RenderKpis(props.averages)
            div {
                chart::Chart(props.measurements)
            }
        },
        Err(_) => view! {
            "Error fetching data."
        },
    }
}
