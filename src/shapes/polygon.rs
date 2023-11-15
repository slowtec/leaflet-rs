use js_sys::Array;
use wasm_bindgen::prelude::*;

use crate::evented::{LeafletEventHandler, MouseEvents, PopupEvents, TooltipEvents};
use crate::{Evented, LatLng, Layer, LayerEvents, Polyline, PolylineOptions};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Polyline)]
    pub type Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlngs: &Array) -> Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlngs: &Array, options: &PolylineOptions) -> Polygon;

    #[wasm_bindgen(method, js_name = getCenter)]
    pub fn get_center(this: &Polygon) -> LatLng;
}

/// Seems that wasmbindgen doesn't implement From<Polyline> for Layer
impl From<Polygon> for Layer {
    fn from(value: Polygon) -> Self {
        value.unchecked_into()
    }
}

impl LeafletEventHandler for Polygon {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MouseEvents for Polygon {}
impl LayerEvents for Polygon {}
impl PopupEvents for Polygon {}
impl TooltipEvents for Polygon {}
