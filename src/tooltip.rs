use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{create_object_with_properties, DivOverlay, LatLng, Layer, Point};

#[wasm_bindgen]
extern "C" {
    /// [`Tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = DivOverlay)]
    pub type Tooltip;

    /// [`L.tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip-l-tooltip)
    #[wasm_bindgen(js_namespace = L, constructor, js_name = Tooltip)]
    pub fn new(options: &TooltipOptions, layer: Option<&Layer>) -> Tooltip;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_lat_lng(lat_lng: &LatLng, options: &TooltipOptions) -> Tooltip;

    #[wasm_bindgen(method, js_name = setContent)]
    pub fn set_content(this: &Tooltip, content: &JsValue) -> Tooltip;

    #[wasm_bindgen(method, js_name = getContent)]
    pub fn get_content(this: &Tooltip) -> JsValue;

    #[wasm_bindgen(method, js_name = setLatLng)]
    pub fn set_lat_lng(this: &Tooltip, latlng: &JsValue) -> Tooltip;

    #[wasm_bindgen(method, js_name = getLatLng)]
    pub fn get_lat_lng(this: &Tooltip) -> LatLng;
}

create_object_with_properties!(
    (TooltipOptions, TooltipOptions),
    (pane, pane, String),
    (direction, direction, String),
    (offset, offset, Point),
    (permanent, permanent, bool),
    (sticky, sticky, bool),
    (opacity, opacity, f64)
);

impl Default for TooltipOptions {
    fn default() -> Self {
        Self::new()
    }
}
