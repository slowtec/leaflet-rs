use crate::{ImageOverlay, LatLngBounds, Layer, create_object_with_properties};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
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

create_object_with_properties!(
    (VideoOverlayOptions, VideoOverlayOptions),
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
    (attribution, attribution, String),
    (autoplay, autoplay, bool),
    (looped, loop, bool),
    (keep_aspect_ratio, keepAspectRatio, bool),
    (muted, muted, bool),
    (plays_inline, playsInline, bool)
);

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
