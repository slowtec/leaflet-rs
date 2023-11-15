use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{LatLng, Layer, Map};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = L, js_name = DivOverlay, extends = Layer)]
    #[derive(Debug, Clone)]
    pub type DivOverlay;

    #[wasm_bindgen(constructor)]
    pub fn new(options: &JsValue) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn setContent(this: &DivOverlay, content: &JsValue) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn getContent(this: &DivOverlay) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &DivOverlay, latlng: &JsValue) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &DivOverlay) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn toggle(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn close(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn isOpen(this: &DivOverlay) -> bool;

    #[wasm_bindgen(method)]
    pub fn getElement(this: &DivOverlay) -> HtmlElement;

    #[wasm_bindgen(method)]
    pub fn update(this: &DivOverlay);

    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn openOn(this: &DivOverlay, map: &Map);
}
