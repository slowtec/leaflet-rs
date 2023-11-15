use crate::{Event, LatLng, LatLngBounds};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = Event, js_name = LocationEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type LocationEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> LocationEvent;

    #[wasm_bindgen(method, getter, js_name = latlng)]
    pub fn lat_lng(this: &LocationEvent) -> LatLng;

    #[wasm_bindgen(method, setter,js_name = latlng)]
    pub fn set_lat_lng(this: &LocationEvent, lat_lng: &LatLng);

    #[wasm_bindgen(method, getter, js_name = bounds)]
    pub fn bounds(this: &LocationEvent) -> LatLngBounds;

    #[wasm_bindgen(method, setter,js_name = bounds)]
    pub fn set_bounds(this: &LocationEvent, value: &LatLngBounds);

    #[wasm_bindgen(method, getter, js_name = accuracy)]
    pub fn accuracy(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = accuracy)]
    pub fn set_accuracy(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = altitude)]
    pub fn altitude(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = altitude)]
    pub fn set_altitude(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = altitudeAccuracy)]
    pub fn altitude_accuracy(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = altitudeAccuracy)]
    pub fn set_altitude_accuracy(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = heading)]
    pub fn heading(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = heading)]
    pub fn set_heading(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = speed)]
    pub fn speed(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = speed)]
    pub fn set_speed(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = timestamp)]
    pub fn timestamp(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = timestamp)]
    pub fn set_timestamp(this: &LocationEvent, value: f64);
}
