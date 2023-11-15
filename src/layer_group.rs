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
    #[wasm_bindgen(method, js_name = toGeoJSON)]
    pub fn to_geo_json(this: &LayerGroup) -> JsValue;

    /// [`addLayer`](https://leafletjs.com/reference.html#layergroup-addlayer)
    #[wasm_bindgen(method, js_name = addLayer)]
    pub fn add_layer(this: &LayerGroup, layer: &Layer) -> LayerGroup;

    /// [`removeLayer`](https://leafletjs.com/reference.html#layergroup-removelayer)
    #[wasm_bindgen(method, js_name = removeLayer)]
    pub fn remove_layer(this: &LayerGroup, layer: &Layer) -> LayerGroup;

    /// [`hasLayer`](https://leafletjs.com/reference.html#layergroup-haslayer)
    #[wasm_bindgen(method, js_name = hasLayer)]
    pub fn has_layer(this: &LayerGroup, layer: &Layer) -> bool;

    /// [`clearLayers`](https://leafletjs.com/reference.html#layergroup-clearlayers)
    #[wasm_bindgen(method, js_name = clearLayers)]
    pub fn clear_layers(this: &LayerGroup) -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn invoke(this: &LayerGroup, method_name: &str) -> LayerGroup;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn each_layer(this: &LayerGroup, callback: &Closure<dyn Fn(Layer)>) -> LayerGroup;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn each_layer_with_context(
        this: &LayerGroup,
        callback: &Closure<dyn Fn(Object, Layer)>,
        context: &Object,
    ) -> LayerGroup;

    #[wasm_bindgen(method, js_name = getLayer)]
    pub fn get_layer(this: &LayerGroup, id: i32) -> Layer;

    #[wasm_bindgen(method, js_name = getLayers)]
    pub fn get_layers(this: &LayerGroup) -> Array;

    #[wasm_bindgen(method, js_name = setZIndex)]
    pub fn set_z_index(this: &LayerGroup, index: f64) -> LayerGroup;

    #[wasm_bindgen(method, js_name = getLayerId)]
    pub fn get_layer_id(this: &LayerGroup, layer: &Layer) -> i32;
}
