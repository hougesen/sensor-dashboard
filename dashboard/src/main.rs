mod components;
mod models;
mod pages;
mod services;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Route)]
enum AppRoutes {
    #[to("/")]
    Dashboard,
    #[not_found]
    NotFound,
}

#[component(App<G>)]
fn app() -> View<G> {
    view! {
        Router(RouterProps::new(HistoryIntegration::new(), |route: ReadSignal<AppRoutes>| {
            let template = Signal::new(View::empty());
            create_effect(cloned!((template) => move || {
                let route = route.get();
                spawn_local(cloned!((template) => async move {
                    let t = match route.as_ref() {
                        AppRoutes::Dashboard  => {
                            let data = services::fetch_average_by_location(1).await;
                            println!("data {:#?}", data);
                            view! {
                                pages::dashboard::Dashboard(data)
                            }
                        },
                        AppRoutes::NotFound => view! {
                            "Page not found."
                        },
                    };
                    template.set(t);
                }));
            }));

            view! {
                div(class="app mb-2 bg-gray-200 w-full h-full min-h-screen") {
                    div(class="container mx-auto") {
                        (template.get().as_ref())
                    }
                }
            }
        }))
    }
}

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init().expect("error initializing logger");

    sycamore::render(|| {
        view! {
            App()
        }
    });
}
