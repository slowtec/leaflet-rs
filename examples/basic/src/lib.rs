use gloo_events::EventListener;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Array, Function};
use leaflet::{
    Circle, CircleOptions, Control, ControlOptions, LatLng, LatLngBounds, Map, MapOptions, Polygon,
    Polyline, PolylineOptions, Rectangle, TileLayer,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, window, Element, HtmlAnchorElement};

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Running Leaflet example code in Rust.".into());

    let options = MapOptions::default();
    let map = Map::new("map", &options);
    map.setView(&LatLng::new(63.5, 10.5), 5.0);

    add_tile_layer(&map);
    add_polyline(&map);
    add_polygon(&map);
    add_rectangle(&map);
    add_circle(&map);
    add_circle_with_options(&map);
    add_control(&map);

    Ok(())
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").addTo(map);
}

fn add_polyline(map: &Map) {
    let options = PolylineOptions::default();
    Polyline::new_with_options(
        &[
            LatLng::new(63.25, 11.25),
            LatLng::new(63.75, 11.75),
            LatLng::new(63.5, 12.0),
        ]
        .iter()
        .map(JsValue::from)
        .collect::<Array>(),
        &options,
    )
    .addTo(map);
}

fn add_polygon(map: &Map) {
    Polygon::new(
        &[
            LatLng::new(63.25, 12.25),
            LatLng::new(63.75, 12.75),
            LatLng::new(63.5, 13.0),
        ]
        .iter()
        .map(JsValue::from)
        .collect::<Array>(),
    )
    .addTo(map);
}

fn add_rectangle(map: &Map) {
    Rectangle::new(&LatLngBounds::new(
        &LatLng::new(63.25, 10.25),
        &LatLng::new(63.75, 10.75),
    ))
    .addTo(map);
}

fn add_circle(map: &Map) {
    Circle::new(&LatLng::new(63.25, 13.25)).addTo(map);
}

fn add_circle_with_options(map: &Map) {
    let mut options = CircleOptions::default();
    options.radius(4000.0);
    Circle::new_with_options(&LatLng::new(63.25, 13.35), &options).addTo(map);
}

fn add_control(map: &Map) {
    let mut options = ControlOptions::default();
    options.position("topleft");
    let control_button = Control::new(&options);

    // This callback must return a HTML div representing the control button.
    let on_add = |_: &_| {
        let document = window()
            .expect("Unable to get browser window")
            .document()
            .expect("Unable to get browser document");

        let container = document
            .create_element("div")
            .expect("Unable to create div");

        container.set_class_name("leaflet-bar");

        let link = document
            .create_element("a")
            .expect("Unable to create link")
            .dyn_into::<HtmlAnchorElement>()
            .expect("Unable to cast to HtmlAnchorElement");

        link.set_href("#");
        link.set_inner_html("⬤");
        link.set_title("Create a new foobar.");

        let on_click = EventListener::new(&link, "click", |_| {
            console::log_1(&"Control button click.".into());
        });

        on_click.forget();

        container
            .append_child(&link)
            .expect("Unable to add child element");

        container.dyn_into().unwrap()
    };

    control_button.on_add(on_add);
    control_button.addTo(map);
}
