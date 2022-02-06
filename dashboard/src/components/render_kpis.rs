use crate::models::*;

use sycamore::prelude::*;

#[component(RenderKpis<G>)]
pub fn render_kpis(kpis: Vec<AverageKpi>) -> View<G> {
    let kpis_view = View::new_fragment(
        kpis.into_iter()
            .map(|kpi| {
                view! {
                    div (class="w-full bg-white rounded py-4 text-center m-4 flex flex-col") {
                        h2 (class="font-bold text-xl") {
                            (format!("{:.2}", kpi.average_value))
                        }
                        p (class="capitalize") {
                            "Avg. " (kpi.measurement_name.replace("_", " "))
                        }
                    }
                }
            })
            .collect(),
    );

    view! {
        div (class="grid grid-cols-4 gap-4") {
            (kpis_view)
        }
    }
}
