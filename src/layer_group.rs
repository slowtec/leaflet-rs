use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use crate::Layer;

#[wasm_bindgen]
extern "C" {
    /// [`LayerGroup`](https://leafletjs.com/reference.html#layergroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type LayerGroup;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> LayerGroup;

    /// [`toGeoJSON`](https://leafletjs.com/reference.html#layergroup-togeojson)
    #[wasm_bindgen(method)]
    pub fn toGeoJSON(this: &LayerGroup) -> JsValue;

    /// [`addLayer`](https://leafletjs.com/reference.html#layergroup-addlayer)
    #[wasm_bindgen(method)]
    pub fn addLayer(this: &LayerGroup, layer: &Layer) -> LayerGroup;

    /// [`removeLayer`](https://leafletjs.com/reference.html#layergroup-removelayer)
    #[wasm_bindgen(method)]
    pub fn removeLayer(this: &LayerGroup, layer: &Layer) -> LayerGroup;

    /// [`hasLayer`](https://leafletjs.com/reference.html#layergroup-haslayer)
    #[wasm_bindgen(method)]
    pub fn hasLayer(this: &LayerGroup, layer: &Layer) -> bool;

    /// [`clearLayers`](https://leafletjs.com/reference.html#layergroup-clearlayers)
    #[wasm_bindgen(method)]
    pub fn clearLayers(this: &LayerGroup) -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn invoke(this: &LayerGroup, method_name: &str) -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn eachLayer(this: &LayerGroup, callback: &Closure<dyn Fn(Layer)>) -> LayerGroup;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn eachLayerWithContext(
        this: &LayerGroup,
        callback: &Closure<dyn Fn(Object, Layer)>,
        context: &Object,
    ) -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn getLayer(this: &LayerGroup, id: i32) -> Layer;

    #[wasm_bindgen(method)]
    pub fn getLayers(this: &LayerGroup) -> Array;

    #[wasm_bindgen(method)]
    pub fn setZIndex(this: &LayerGroup, index: f64) -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn getLayerId(this: &LayerGroup, layer: &Layer) -> i32;
}
