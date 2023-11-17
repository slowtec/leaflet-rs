use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{create_object_with_properties, LatLngBounds, Layer, LayerOptions, Point};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Layer)]
    #[derive(Debug, Clone, PartialEq)]
    pub type GridLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> GridLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(options: &GridLayerOptions) -> GridLayer;

    #[wasm_bindgen(method, js_name = bringToFront)]
    pub fn bring_to_front(this: &GridLayer) -> GridLayer;

    #[wasm_bindgen(method, js_name = bringToBack)]
    pub fn bring_to_back(this: &GridLayer) -> GridLayer;

    #[wasm_bindgen(method, js_name = getContainer)]
    pub fn get_container(this: &GridLayer) -> HtmlElement;

    #[wasm_bindgen(method, js_name = setOpacity)]
    pub fn set_opacity(this: &GridLayer, opacity: f64) -> GridLayer;

    #[wasm_bindgen(method, js_name = setZIndex)]
    pub fn set_z_index(this: &GridLayer, opacity: f64) -> GridLayer;

    #[wasm_bindgen(method, js_name = isLoading)]
    pub fn is_loading(this: &GridLayer) -> bool;

    #[wasm_bindgen(method, js_name = redraw)]
    pub fn redraw(this: &GridLayer) -> GridLayer;
}

create_object_with_properties!(
    (GridLayerOptions, GridLayerOptions, LayerOptions),
    (tile_size, tileSize, f64),
    (tile_size_point, tileSize, Point),
    (opacity, opacity, f64),
    (update_when_idle, updateWhenIdle, bool),
    (update_when_zooming, updateWhenZooming, bool),
    (update_interval, updateInterval, f64),
    (z_index, zIndex, f64),
    (bounds, bounds, LatLngBounds),
    (min_zoom, minZoom, f64),
    (max_zoom, maxZoom, f64),
    (min_native_zoom, minNativeZoom, f64),
    (max_native_zoom, maxNativeZoom, f64),
    (no_wrap, noWrap, bool),
    (pane, pane, String),
    (class_name, className, String),
    (keep_buffer, keepBuffer, f64)
);

impl Default for GridLayerOptions {
    fn default() -> Self {
        GridLayerOptions::new()
    }
}
