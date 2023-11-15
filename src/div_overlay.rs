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

    #[wasm_bindgen(method, js_name = "setContent")]
    pub fn set_content(this: &DivOverlay, content: &JsValue) -> DivOverlay;

    #[wasm_bindgen(method, js_name = "getContent")]
    pub fn get_content(this: &DivOverlay) -> JsValue;

    #[wasm_bindgen(method, js_name = "setLatLng")]
    pub fn set_lat_lng(this: &DivOverlay, latlng: &JsValue) -> DivOverlay;

    #[wasm_bindgen(method, js_name = "getLatLng")]
    pub fn get_lat_lng(this: &DivOverlay) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn toggle(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method)]
    pub fn close(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method, js_name = "isOpen")]
    pub fn is_open(this: &DivOverlay) -> bool;

    #[wasm_bindgen(method, js_name = "getElement")]
    pub fn get_element(this: &DivOverlay) -> HtmlElement;

    #[wasm_bindgen(method)]
    pub fn update(this: &DivOverlay);

    #[wasm_bindgen(method, js_name = "bringToFront")]
    pub fn bring_to_front(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method, js_name = "bringToBack")]
    pub fn bring_to_back(this: &DivOverlay) -> DivOverlay;

    #[wasm_bindgen(method, js_name = "openOn")]
    pub fn open_on(this: &DivOverlay, map: &Map);
}
