use crate::{object_constructor, object_property_set, LatLngBounds, Layer};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Object, js_name = ImageOverlayOptions)]
    pub type ImageOverlayOptions;

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

impl ImageOverlayOptions {
    object_constructor!();
    object_property_set!(opacity, f64);
    object_property_set!(alt, &str);
    object_property_set!(interactive, bool);
    object_property_set!(cross_origin, crossOrigin, &str);
    object_property_set!(cross_origin_toggle, crossOrigin, bool);
    object_property_set!(error_overlay_url, errorOverlayUrl, &str);
    object_property_set!(z_index, zIndex, f64);
    object_property_set!(class_name, className, &str);
    // Interactive layer
    object_property_set!(bubbling_mouse_events, bubblingMouseEvents, bool);
    // Layer options
    object_property_set!(pane, &str);
    object_property_set!(attribution, &str);
}

impl Default for ImageOverlayOptions {
    fn default() -> Self {
        ImageOverlayOptions::new()
    }
}
