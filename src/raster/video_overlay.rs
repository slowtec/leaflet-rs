use crate::{object_constructor, object_property_set, ImageOverlay, LatLngBounds, Layer};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Object, js_name = ImageOverlayOptions)]
    pub type VideoOverlayOptions;

    #[wasm_bindgen(extends = ImageOverlay, js_name = ImageOverlay, js_namespace = L)]
    #[derive(Debug, Clone)]
    pub type VideoOverlay;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(image_url: &str, bounds: &LatLngBounds) -> VideoOverlay;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(
        image_url: &str,
        bounds: &LatLngBounds,
        options: &VideoOverlayOptions,
    ) -> VideoOverlay;
}

impl VideoOverlayOptions {
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
    object_property_set!(autoplay, bool);
    object_property_set!(looped, loop, bool);
    object_property_set!(keep_aspect_ratio, keepAspectRatio, bool);
    object_property_set!(muted, bool);
    object_property_set!(plays_inline, playsInline, bool);
}

impl Default for VideoOverlayOptions {
    fn default() -> Self {
        VideoOverlayOptions::new()
    }
}

impl From<VideoOverlay> for Layer {
    fn from(value: VideoOverlay) -> Self {
        value.unchecked_into()
    }
}
