use wasm_bindgen::prelude::*;

use crate::evented::{LeafletEventHandler, MouseEvents, PopupEvents, TooltipEvents};
use crate::{Evented, LatLngBounds, LayerEvents, Polygon, PolylineOptions};

#[wasm_bindgen]
extern "C" {
    // Rectangle

    #[derive(Debug)]
    #[wasm_bindgen(extends = Polygon)]
    pub type Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(bounds: &LatLngBounds) -> Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(bounds: &LatLngBounds, options: &PolylineOptions) -> Rectangle;

    #[wasm_bindgen(method)]
    pub fn setBounds(this: &Rectangle, bounds: &LatLngBounds) -> Rectangle;
}

impl LeafletEventHandler for Rectangle {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MouseEvents for Rectangle {}
impl LayerEvents for Rectangle {}
impl PopupEvents for Rectangle {}
impl TooltipEvents for Rectangle {}
