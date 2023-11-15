use wasm_bindgen::prelude::*;

use crate::{LayerGroup, LatLngBounds};

#[wasm_bindgen]
extern "C" {
    /// [`FeatureGroup`](https://leafletjs.com/reference-1.7.1.html#featuregroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = LayerGroup)]
    pub type FeatureGroup;

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#featuregroup-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &FeatureGroup, style: &JsValue);

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &FeatureGroup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &FeatureGroup);

    /// [`getBounds`](https://leafletjs.com/reference-1.7.1.html#featuregroup-getbounds)
    #[wasm_bindgen(method)]
    pub fn getBounds(this: &FeatureGroup) -> LatLngBounds;

}
