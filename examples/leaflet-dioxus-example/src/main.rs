#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use gloo_utils::document;
use leaflet::{Map, MapOptions, Marker, TileLayer};
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

#[component]
fn App() -> Element {
    use_effect(move || {
        const GREENWICH_MERIDIAN: (f64, f64) = (51.477806, -0.001472);

        let container: HtmlElement = document()
            .get_element_by_id("map")
            .unwrap()
            .dyn_into()
            .unwrap();

        let map = Map::new_with_element(&container, &MapOptions::default());

        TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);

        map.set_view(&GREENWICH_MERIDIAN.into(), 14.0);

        Marker::new(&GREENWICH_MERIDIAN.into()).add_to(&map);
    });

    rsx! {
        div { id: "map" }
    }
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}
