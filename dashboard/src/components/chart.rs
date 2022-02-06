use crate::models::Measurement;
use sycamore::prelude::{component, view, View};

#[component(Chart<G>)]
pub fn chart(values: Vec<Measurement>) -> View<G> {
    let length = values.len();

    let kpis_view = View::new_fragment({
        let mut max_value: f32 = 0.0;

        for i in 0..values.len() {
            if values[i].measurement_value > max_value {
                max_value = values[i].measurement_value;
            }
        }

        let multiplier: f32 = 100 as f32 / max_value;

        values
            .into_iter()
            .map(|measurement| {
                view! {
                    div (class="ml-1 mt-auto", title=format!("{} - {}", &measurement.measurement_value, &measurement.measurement_time)) {
                        div (class = "w-full bg-black p-1", style = format!("height: {}px", measurement.measurement_value * multiplier))
                    }
                }
            })
            .collect()
    });

    view! {
        div (class="grid w-full h-full", style=format!("grid-template-columns: repeat({}, 1fr)", length)) {
            (kpis_view)
        }
    }
}
