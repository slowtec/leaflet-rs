use wasm_bindgen::prelude::*;

use crate::{LatLng, Point};

#[wasm_bindgen]
extern "C" {
    // CRS
    #[derive(Debug)]
    pub type Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Simple")]
    pub fn new_simple() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Earth")]
    pub fn new_earth() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG3395")]
    pub fn new_epsg_3395() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG3857")]
    pub fn new_epsg_3857() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG4326")]
    pub fn new_epsg_4326() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Base")]
    pub fn new_base() -> Crs;

    #[wasm_bindgen(method, js_name = "latLngToPoint")]
    pub fn lat_lng_to_point(this: &Crs, latlng: LatLng, zoom: f32) -> Point;
}
