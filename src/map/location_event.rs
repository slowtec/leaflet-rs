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
    pub fn latlng(this: &LocationEvent) -> LatLng;

    #[wasm_bindgen(method, setter,js_name = latlng)]
    pub fn setLatlng(this: &LocationEvent, lat_lng: &LatLng);

    #[wasm_bindgen(method, getter, js_name = bounds)]
    pub fn bounds(this: &LocationEvent) -> LatLngBounds;

    #[wasm_bindgen(method, setter,js_name = bounds)]
    pub fn setBounds(this: &LocationEvent, value: &LatLngBounds);

    #[wasm_bindgen(method, getter, js_name = accuracy)]
    pub fn accuracy(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = accuracy)]
    pub fn setAccuracy(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = altitude)]
    pub fn altitude(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = altitude)]
    pub fn setAltitude(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = altitudeAccuracy)]
    pub fn altitudeAccuracy(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = altitudeAccuracy)]
    pub fn setAltitudeAccuracy(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = heading)]
    pub fn heading(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = heading)]
    pub fn setHeading(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = speed)]
    pub fn speed(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = speed)]
    pub fn setSpeed(this: &LocationEvent, value: f64);

    #[wasm_bindgen(method, getter, js_name = timestamp)]
    pub fn timestamp(this: &LocationEvent) -> f64;

    #[wasm_bindgen(method, setter,js_name = timestamp)]
    pub fn setTimestamp(this: &LocationEvent, value: f64);
}
