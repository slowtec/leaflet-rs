use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{object_constructor, object_property_set, DivOverlay, LatLng, Layer, Point};

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Object, js_name = PopupOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type TooltipOptions;

    /// [`Tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = DivOverlay)]
    pub type Tooltip;

    /// [`L.tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip-l-tooltip)
    #[wasm_bindgen(js_namespace = L, constructor, js_name = Tooltip)]
    pub fn new(options: &TooltipOptions, layer: Option<&Layer>) -> Tooltip;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn newWithLatLng(lat_lng: &LatLng, options: &TooltipOptions) -> Tooltip;

    #[wasm_bindgen(method)]
    pub fn setContent(this: &Tooltip, content: &JsValue) -> Tooltip;

    #[wasm_bindgen(method)]
    pub fn getContent(this: &Tooltip) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Tooltip, latlng: &JsValue) -> Tooltip;

    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Tooltip) -> LatLng;
}

impl TooltipOptions {
    object_constructor!();
    object_property_set!(pane, &str);
    object_property_set!(direction, &str);
    object_property_set!(offset, &Point);
    object_property_set!(permanent, bool);
    object_property_set!(sticky, bool);
    object_property_set!(opacity, f64);
}

impl Default for TooltipOptions {
    fn default() -> Self {
        Self::new()
    }
}
