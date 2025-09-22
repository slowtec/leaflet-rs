use crate::{LatLngBounds, Layer, create_object_with_properties};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Layer, js_name = ImageOverlay, js_namespace = L)]
    #[derive(Debug, Clone)]
    pub type ImageOverlay;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(image_url: &str, bounds: &LatLngBounds) -> ImageOverlay;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(
        image_url: &str,
        bounds: &LatLngBounds,
        options: &ImageOverlayOptions,
    ) -> ImageOverlay;
}

create_object_with_properties!(
    (ImageOverlayOptions, ImageOverlayOptions),
    (opacity, opacity, f64),
    (alt, alt, String),
    (interactive, interactive, bool),
    (cross_origin, crossOrigin, String),
    (cross_origin_toggle, crossOrigin, bool),
    (error_overlay_url, errorOverlayUrl, String),
    (z_index, zIndex, f64),
    (class_name, className, String),
    // Interactive layer
    (bubbling_mouse_events, bubblingMouseEvents, bool),
    // Layer options
    (pane, pane, String),
    (attribution, attribution, String)
);

impl Default for ImageOverlayOptions {
    fn default() -> Self {
        ImageOverlayOptions::new()
    }
}
