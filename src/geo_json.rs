use wasm_bindgen::prelude::*;

use crate::Layer;

#[wasm_bindgen]
extern "C" {
    /// [`GeoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type GeoJSON;

    /// [`L.geoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson-l-geojson)
    #[wasm_bindgen(js_namespace = L)]
    pub fn geoJSON(geojson: &JsValue, options: &JsValue) -> GeoJSON;

    /// [`addData`](https://leafletjs.com/reference-1.7.1.html#geojson-adddata)
    #[wasm_bindgen(method)]
    pub fn addData(this: &GeoJSON, data: &JsValue);

    /// [`resetStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-resetstyle)
    #[wasm_bindgen(method)]
    pub fn resetStyle(this: &GeoJSON, layer: Option<&Layer>);

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &GeoJSON, style: &JsValue);
}
