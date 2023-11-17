use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{create_object_with_properties, GridLayer, GridLayerOptions, LatLng};

#[wasm_bindgen]
extern "C" {

    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = GridLayer)]
    pub type TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(url_template: &str) -> TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_options(url_template: &str, options: &TileLayerOptions) -> TileLayer;

    #[wasm_bindgen(method, js_name = setUrl)]
    pub fn set_url(this: &TileLayer, url: &str, no_redraw: Option<bool>) -> TileLayer;

    #[wasm_bindgen(method, getter, js_name = _url)]
    pub fn url(this: &TileLayer) -> String;

    #[wasm_bindgen(method, js_name = getTileUrl)]
    pub fn get_tile_url(this: &TileLayer, coords: &LatLng) -> String;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn create_tile(this: &TileLayer, lat_long: &LatLng) -> HtmlElement;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn create_tile_with_done(
        this: &TileLayer,
        lat_long: &LatLng,
        done: &Function,
    ) -> HtmlElement;
}

create_object_with_properties!(
    (TileLayerOptions, TileLayerOptions, GridLayerOptions),
    (min_zoom, minZoom, f64),
    (max_zoom, maxZoom, f64),
    (subdomains, subdomains, bool),
    (error_tile_url, errorTileUrl, String),
    (zoom_offset, zoomOffset, f64),
    (tms, tms, bool),
    (zoom_reverse, zoomReverse, bool),
    (detect_retina, detectRetina, bool),
    (cross_origin, crossOrigin, String),
    (referrer_policy, referrerPolicy, String)
);

impl Default for TileLayerOptions {
    fn default() -> Self {
        TileLayerOptions::new()
    }
}
