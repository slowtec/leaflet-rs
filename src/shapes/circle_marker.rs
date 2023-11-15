use wasm_bindgen::prelude::*;

use crate::evented::{LeafletEventHandler, MouseEvents, MoveEvents, PopupEvents, TooltipEvents};
use crate::{Evented, LatLng, LayerEvents, Path};

#[wasm_bindgen]
extern "C" {
    // CircleMarker

    /// [`CircleMarker`](https://leafletjs.com/reference-1.7.1.html#circlemarker)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Path)]
    pub type CircleMarker;

    /// [`Constructor`](https://leafletjs.com/reference-1.7.1.html#circlemarker-l-circlemarker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> CircleMarker;

    /// [`Constructor`](https://leafletjs.com/reference-1.7.1.html#circlemarker-l-circlemarker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> CircleMarker;

    /// [`toGeoJSON`](https://leafletjs.com/reference-1.7.1.html#circlemarker-togeojson)
    #[wasm_bindgen(method)]
    pub fn toGeoJSON(this: &CircleMarker) -> JsValue;

    /// [`setLatLng`](https://leafletjs.com/reference-1.7.1.html#circlemarker-setlanglng)
    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &CircleMarker, latlng: &LatLng);

    /// [`getLatLng`](https://leafletjs.com/reference-1.7.1.html#circlemarker-getlatlng)
    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &CircleMarker) -> LatLng;

    /// [`setRadius`](https://leafletjs.com/reference-1.7.1.html#circlemarker-setradius)
    #[wasm_bindgen(method)]
    pub fn setRadius(this: &CircleMarker, radius: f64);

    /// [`getRadius`](https://leafletjs.com/reference-1.7.1.html#circlemarker-getradius)
    #[wasm_bindgen(method)]
    pub fn getRadius(this: &CircleMarker) -> f64;

}

impl LeafletEventHandler for CircleMarker {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MoveEvents for CircleMarker {}
impl MouseEvents for CircleMarker {}
impl LayerEvents for CircleMarker {}
impl PopupEvents for CircleMarker {}
impl TooltipEvents for CircleMarker {}
