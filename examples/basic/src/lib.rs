use gloo_events::EventListener;
use js_sys::{Array, Function};
use leaflet::{
    Circle, Control, LatLng, LatLngBounds, Map, Polygon, Polyline, Rectangle, TileLayer,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Element, HtmlAnchorElement};

#[derive(Serialize, Deserialize)]
struct CircleOptions {
    radius: f32,
}

#[derive(Serialize, Deserialize)]
struct PolylineOptions {
    color: String,
}

#[derive(Serialize, Deserialize)]
struct ControlOptions {
    position: String,
}

#[derive(Serialize, Deserialize)]
struct ControlProps {
    options: ControlOptions,
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Running Leaflet example code in Rust.".into());

    let map = Map::new("map", &JsValue::NULL);
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
    TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}

fn add_polyline(map: &Map) {
    Polyline::new_with_options(
        [
            LatLng::new(63.25, 11.25),
            LatLng::new(63.75, 11.75),
            LatLng::new(63.5, 12.0),
        ]
        .iter()
        .map(JsValue::from)
        .collect(),
        &JsValue::from_serde(&PolylineOptions {
            color: "red".into(),
        })
        .expect("Unable to serialize polyline options"),
    )
    .addTo(&map);
}

fn add_polygon(map: &Map) {
    Polygon::new(
        [
            LatLng::new(63.25, 12.25),
            LatLng::new(63.75, 12.75),
            LatLng::new(63.5, 13.0),
        ]
        .iter()
        .map(JsValue::from)
        .collect(),
    )
    .addTo(&map);
}

fn add_rectangle(map: &Map) {
    Rectangle::new(&LatLngBounds::new(
        &LatLng::new(63.25, 10.25),
        &LatLng::new(63.75, 10.75),
    ))
    .addTo(&map);
}

fn add_circle(map: &Map) {
    Circle::new(&LatLng::new(63.25, 13.25)).addTo(&map);
}

fn add_circle_with_options(map: &Map) {
    Circle::new_with_options(
        &LatLng::new(63.25, 13.35),
        &JsValue::from_serde(&CircleOptions { radius: 4000.0 })
            .expect("Unable to serialize circle options"),
    )
    .addTo(&map);
}

fn add_control(map: &Map) {
    let props = JsValue::from_serde(&ControlProps {
        options: ControlOptions {
            position: "topleft".into(),
        },
    })
    .expect("Unable to serialize control props");

    // This callback must return a HTML div representing the control button.
    let on_add = || {
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

        container
    };

    let on_add_closure = Closure::wrap(Box::new(on_add) as Box<dyn FnMut() -> Element>);

    js_sys::Reflect::set(&props, &JsValue::from("onAdd"), on_add_closure.as_ref())
        .expect("Unable to set onAdd()");

    on_add_closure.forget();

    let control_class = Control::extend(&props)
        .dyn_into::<Function>()
        .expect("Unable to cast to Function");

    let control_button: Control = JsCast::unchecked_into(
        js_sys::Reflect::construct(&control_class, &Array::new())
            .expect("Unable to run constructor"),
    );

    control_button.addTo(&map);
}
