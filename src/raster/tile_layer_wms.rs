use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{object_constructor, object_property_set, Crs, LatLngBounds, Point, TileLayer};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen (extends = Object , js_name = TileLayerWmsOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = TileLayer)]
    pub type TileLayerWmsOptions;

    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = TileLayer,js_namespace = ["L", "tileLayer"],  js_name = "TileLayerWMS")]
    pub type TileLayerWms;

    #[wasm_bindgen(js_namespace = ["L", "tileLayer"], js_name = "wms")]
    fn new_wms(url_template: &str) -> TileLayerWms;

    #[wasm_bindgen(js_namespace = ["L", "tileLayer"], js_name = "wms")]
    fn new_wms_options(url_template: &str, options: &TileLayerWmsOptions) -> TileLayerWms;

    #[wasm_bindgen(method, js_name = setParams)]
    pub fn setParams(
        this: &TileLayer,
        params: &TileLayerWmsOptions,
        no_redraw: Option<bool>,
    ) -> TileLayerWms;
}

impl TileLayerWmsOptions {
    object_constructor!();
    // TileLayerWmsOptions
    object_property_set!(layers, layers, &str);
    object_property_set!(styles, styles, &str);
    object_property_set!(format, format, &str);
    object_property_set!(transparent, transparent, bool);
    object_property_set!(version, version, &str);
    object_property_set!(crs, crs, Crs);
    object_property_set!(uppercase, uppercase, Crs);
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

impl TileLayerWms {
    pub fn new(url_template: &str) -> TileLayerWms {
        new_wms(url_template)
    }

    pub fn new_options(url_template: &str, options: &TileLayerWmsOptions) -> TileLayerWms {
        new_wms_options(url_template, options)
    }
}

impl Default for TileLayerWmsOptions {
    fn default() -> Self {
        TileLayerWmsOptions::new()
    }
}
