use std::ops::DerefMut;

use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use crate::evented::{LeafletEventHandler, MouseEvents, PopupEvents, TooltipEvents};
use crate::{
    create_object_with_properties, Evented, LatLng, LatLngBounds, Layer, LayerEvents, Path,
    PathOptions, Point,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Path)]
    #[derive(Debug, Clone)]
    pub type Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlngs: &Array) -> Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlngs: &Array, options: &PolylineOptions) -> Polyline;

    #[wasm_bindgen(method, js_name = toGeoJSON)]
    pub fn to_geo_json(this: &Polyline, precision: f64) -> Object;

    #[wasm_bindgen(method, js_name = getLatLngs)]
    pub fn get_lat_lngs(this: &Polyline) -> Array;

    #[wasm_bindgen(method, js_name = setLatLngs)]
    pub fn set_lat_lngs(this: &Polyline, lat_lngs: &Array) -> Polyline;

    #[wasm_bindgen(method, js_name = isEmpty)]
    pub fn is_empty(this: &Polyline) -> bool;

    #[wasm_bindgen(method, js_name = closestLayerPoint)]
    pub fn closest_layer_point(this: &Polyline, point: &Point) -> Point;

    #[wasm_bindgen(method, js_name = getCenter)]
    pub fn get_center(this: &Polyline) -> LatLng;

    #[wasm_bindgen(method, js_name = getBounds)]
    pub fn get_bounds(this: &Polyline) -> LatLngBounds;

    #[wasm_bindgen(method,js_name = addLatLng)]
    pub fn add_lat_lng(this: &Polyline, lat_lng: &LatLng) -> Polyline;
}

create_object_with_properties!(
    (PolylineOptions, PolylineOptions, PathOptions),
    (smooth_factor, smoothFactor, f64),
    (no_clip, noClip, bool)
);

impl Default for PolylineOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Seems that wasmbindgen doesn't implement From<Polyline> for Layer
impl From<Polyline> for Layer {
    fn from(value: Polyline) -> Self {
        value.unchecked_into()
    }
}

impl DerefMut for PolylineOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl LeafletEventHandler for Polyline {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MouseEvents for Polyline {}
impl LayerEvents for Polyline {}
impl PopupEvents for Polyline {}
impl TooltipEvents for Polyline {}
