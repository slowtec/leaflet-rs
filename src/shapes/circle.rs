use js_sys::Object;
use std::ops::DerefMut;
use wasm_bindgen::prelude::*;

use crate::evented::{LeafletEventHandler, MouseEvents, MoveEvents, PopupEvents, TooltipEvents};
use crate::{
    object_constructor, object_property_set, CircleMarker, Evented, LatLng, LatLngBounds, Layer,
    LayerEvents, PathOptions,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = PathOptions, js_name = CircleOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type CircleOptions;

    /// [`Circle`](https://leafletjs.com/reference.html#circle)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = CircleMarker)]
    pub type Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_radius(latlng: &LatLng, radius: f64) -> Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &CircleOptions) -> Circle;

    #[wasm_bindgen(method)]
    pub fn setStyle(this: &Circle, options: &CircleOptions) -> Circle;
    /// [`setRadius`](https://leafletjs.com/reference-1.7.1.html#circle-setradius)
    #[wasm_bindgen(method)]
    pub fn setRadius(this: &Circle, radius: f64);

    /// [`getRadius`](https://leafletjs.com/reference-1.7.1.html#circle-getradius)
    #[wasm_bindgen(method)]
    pub fn getRadius(this: &Circle) -> f64;

    /// [`getBounds`](https://leafletjs.com/reference.html#circle-getbounds)
    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Circle) -> LatLngBounds;
}

impl CircleOptions {
    object_constructor!();
    object_property_set!(radius, f64);
}

impl Default for CircleOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl DerefMut for CircleOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}

impl From<Circle> for Layer {
    fn from(circle: Circle) -> Self {
        circle.unchecked_into()
    }
}

impl LeafletEventHandler for Circle {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MoveEvents for Circle {}
impl MouseEvents for Circle {}
impl LayerEvents for Circle {}
impl PopupEvents for Circle {}
impl TooltipEvents for Circle {}
