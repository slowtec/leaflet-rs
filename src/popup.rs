use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{create_object_with_properties, DivOverlay, LatLng, Layer, Point};

#[wasm_bindgen]
extern "C" {
    // Popup
    /// [`Popup`](https://leafletjs.com/reference.html#popup)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = DivOverlay)]
    pub type Popup;

    /// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &PopupOptions, layer: Option<&Layer>) -> Popup;

    /// [`L.popup`](/// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup))
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_lat_lng(lat_lng: &LatLng, options: &PopupOptions) -> Popup;
}

create_object_with_properties!(
    (PopupOptions, PopupOptions),
    (pane, pane, String),
    (offset, offset, Point),
    (min_width, minWidth, f64),
    (max_width, maxWidth, f64),
    (max_height, maxHeight, f64),
    (auto_pan, autoPan, bool),
    (auto_pan_padding_top_left, autoPanPaddingTopLeft, Point),
    (
        auto_pan_padding_bottom_right,
        autoPanPaddingBottomRight,
        Point
    ),
    (auto_pan_padding, autoPanPadding, Point),
    (keep_in_view, keepInView, bool),
    (close_button, closeButton, bool),
    (auto_close, autoClose, bool),
    (close_on_escape_key, closeOnEscapeKey, bool),
    (close_on_click, closeOnClick, bool),
    (class_name, className, String)
);

impl Default for PopupOptions {
    fn default() -> Self {
        Self::new()
    }
}
