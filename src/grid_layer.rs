use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{object_constructor, object_property_set, LatLngBounds, Layer, Point};

#[wasm_bindgen]
extern "C" {

    # [wasm_bindgen (extends = Object, js_name = GridLayerOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type GridLayerOptions;

    #[wasm_bindgen(extends = Layer)]
    #[derive(Debug, Clone, PartialEq)]
    pub type GridLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> GridLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn newWithOptions(options: &GridLayerOptions) -> GridLayer;

    #[wasm_bindgen(method, js_name = bringToFront)]
    pub fn bringToFront(this: &GridLayer) -> GridLayer;

    #[wasm_bindgen(method, js_name = bringToBack)]
    pub fn bringToBack(this: &GridLayer) -> GridLayer;

    #[wasm_bindgen(method, js_name = getContainer)]
    pub fn getContainer(this: &GridLayer) -> HtmlElement;

    #[wasm_bindgen(method, js_name = setOpacity)]
    pub fn setOpacity(this: &GridLayer, opacity: f64) -> GridLayer;

    #[wasm_bindgen(method, js_name = setZIndex)]
    pub fn setZIndex(this: &GridLayer, opacity: f64) -> GridLayer;

    #[wasm_bindgen(method, js_name = isLoading)]
    pub fn isLoading(this: &GridLayer) -> bool;

    #[wasm_bindgen(method, js_name = redraw)]
    pub fn redraw(this: &GridLayer) -> GridLayer;
}

impl GridLayerOptions {
    object_constructor!();
    object_property_set!(tile_size, tileSize, f64);
    object_property_set!(tile_size_point, tileSize, &Point);
    object_property_set!(opacity, f64);
    object_property_set!(update_when_idle, updateWhenIdle, bool);
    object_property_set!(update_when_zooming, updateWhenZooming, bool);
    object_property_set!(update_interval, updateInterval, f64);
    object_property_set!(z_index, zIndex, f64);
    object_property_set!(bounds, &LatLngBounds);
    object_property_set!(min_zoom, minZoom, f64);
    object_property_set!(max_zoom, maxZoom, f64);
    object_property_set!(min_native_zoom, minNativeZoom, f64);
    object_property_set!(max_native_zoom, maxNativeZoom, f64);
    object_property_set!(no_wrap, noWrap, bool);
    object_property_set!(pane, &str);
    object_property_set!(class_name, className, &str);
    object_property_set!(keep_buffer, keepBuffer, f64);
}

impl Default for GridLayerOptions {
    fn default() -> Self {
        GridLayerOptions::new()
    }
}
