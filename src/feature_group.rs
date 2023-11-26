use wasm_bindgen::prelude::*;

use crate::{LatLngBounds, LayerGroup};

#[wasm_bindgen]
extern "C" {
    /// [`FeatureGroup`](https://leafletjs.com/reference-1.7.1.html#featuregroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = LayerGroup)]
    pub type FeatureGroup;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> FeatureGroup;

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#featuregroup-setstyle)
    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &FeatureGroup, style: &JsValue);

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtofront)
    #[wasm_bindgen(method, js_name = "bringToFront")]
    pub fn bring_to_front(this: &FeatureGroup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtoback)
    #[wasm_bindgen(method, js_name = "bringToBack")]
    pub fn bring_to_back(this: &FeatureGroup);

    /// [`getBounds`](https://leafletjs.com/reference-1.7.1.html#featuregroup-getbounds)
    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &FeatureGroup) -> LatLngBounds;

}
