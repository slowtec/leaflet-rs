use wasm_bindgen::prelude::*;

use crate::{Array, LatLng};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type LatLngBounds;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(corner1: &LatLng, corner2: &LatLng) -> LatLngBounds;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_from_list(positions: &Array) -> LatLngBounds;

    #[wasm_bindgen(method, js_namespace = L)]
    pub fn extend(this: &LatLngBounds, latlng: &LatLng) -> LatLngBounds;

    #[wasm_bindgen(method, js_namespace = L, js_name = extend)]
    pub fn extend_with_bounds(this: &LatLngBounds, bounds: &LatLngBounds) -> LatLngBounds;

    #[wasm_bindgen(method, js_name = getNorthEast)]
    pub fn get_north_east(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method, js_name = getNorthWest)]
    pub fn get_north_west(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method, js_name = getSouthEast)]
    pub fn get_south_east(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method, js_name = getSouthWest)]
    pub fn get_south_west(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn contains(this: &LatLngBounds, latlng: &LatLng) -> bool;

    #[wasm_bindgen(method, js_name = toBBoxString)]
    pub fn to_bbox_string(this: &LatLngBounds) -> String;
}

impl From<(LatLng, LatLng)> for LatLngBounds {
    fn from(value: (LatLng, LatLng)) -> LatLngBounds {
        LatLngBounds::new(&value.0, &value.1)
    }
}
