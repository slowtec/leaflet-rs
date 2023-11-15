use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{object_constructor, object_property_set, GridLayer, LatLng, LatLngBounds, Point};

#[wasm_bindgen]
extern "C" {

    # [wasm_bindgen (extends = Object , js_name = TileLayerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = GridLayer)]
    pub type TileLayerOptions;

    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = GridLayer)]
    pub type TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(url_template: &str) -> TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_options(url_template: &str, options: &TileLayerOptions) -> TileLayer;

    #[wasm_bindgen(method, js_name = setUrl)]
    pub fn setUrl(this: &TileLayer, url: &str, no_redraw: Option<bool>) -> TileLayer;

    #[wasm_bindgen(method, js_name = getTileUrl)]
    pub fn getTileUrl(this: &TileLayer, coords: &LatLng) -> String;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn createTile(this: &TileLayer, lat_long: &LatLng) -> HtmlElement;

    #[wasm_bindgen(method, js_name = createTile)]
    pub fn createTileWithDone(this: &TileLayer, lat_long: &LatLng, done: &Function) -> HtmlElement;
}

impl TileLayerOptions {
    object_constructor!();
    // TileLayerOptions
    object_property_set!(min_zoom, minZoom, f64);
    object_property_set!(max_zoom, maxZoom, f64);
    object_property_set!(subdomains, bool);
    object_property_set!(error_tile_url, errorTileUrl, &str);
    object_property_set!(zoom_offset, zoomOffset, f64);
    object_property_set!(tms, bool);
    object_property_set!(zoom_reverse, zoomReverse, bool);
    object_property_set!(detect_retina, detectRetina, bool);
    object_property_set!(cross_origin, crossOrigin, &str);
    object_property_set!(referrer_policy, referrerPolicy, &str);
    // GridLayerOptions
    object_property_set!(tile_size, tileSize, u32);
    object_property_set!(tile_size_with_point, tileSize, Point);
    object_property_set!(opacity, opacity, f64);
    object_property_set!(update_when_idle, updateWhenIdle, bool);
    object_property_set!(update_when_zooming, updateWhenZooming, bool);
    object_property_set!(update_interval, updateInterval, f64);
    object_property_set!(z_index, zIndex, f64);
    object_property_set!(bounds, bounds, LatLngBounds);
    object_property_set!(max_native_zoom, maxNativeZoom, f64);
    object_property_set!(min_native_zoom, minNativeZoom, f64);
    object_property_set!(no_wrap, noWrap, bool);
    object_property_set!(pane, &str);
    object_property_set!(class_name, className, &str);
    object_property_set!(keep_buffer, keepBuffer, u32);
    // LayerOptions
    object_property_set!(attribution, &str);
}

impl Default for TileLayerOptions {
    fn default() -> Self {
        TileLayerOptions::new()
    }
}
