use wasm_bindgen::prelude::*;

use crate::LatLng;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type LatLngBounds;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(corner1: &LatLng, corner2: &LatLng) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getNorthEast(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getSouthWest(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn contains(this: &LatLngBounds, latlng: &LatLng) -> bool;
}

impl From<(LatLng, LatLng)> for LatLngBounds {
    fn from(value: (LatLng, LatLng)) -> LatLngBounds {
        LatLngBounds::new(&value.0, &value.1)
    }
}
