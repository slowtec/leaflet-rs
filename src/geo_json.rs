use wasm_bindgen::prelude::*;

use crate::Layer;

#[wasm_bindgen]
extern "C" {
    /// [`GeoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer, js_name = "geoJSON")]
    pub type GeoJson;

    /// [`L.geoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson-l-geojson)
    #[wasm_bindgen(js_namespace = L, js_name = "geoJSON")]
    pub fn geo_json(geojson: &JsValue, options: &JsValue) -> GeoJson;

    /// [`addData`](https://leafletjs.com/reference-1.7.1.html#geojson-adddata)
    #[wasm_bindgen(method, js_name = "addData")]
    pub fn add_data(this: &GeoJson, data: &JsValue);

    /// [`resetStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-resetstyle)
    #[wasm_bindgen(method, js_name = "resetStyle")]
    pub fn reset_style(this: &GeoJson, layer: Option<&Layer>);

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-setstyle)
    #[wasm_bindgen(method, js_name = "setStyle")]
    pub fn set_style(this: &GeoJson, style: &JsValue);
}
