use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{object_constructor, object_property_set, DivOverlay, LatLng, Layer, Point};

#[wasm_bindgen]
extern "C" {
    // Popup

    # [wasm_bindgen (extends = Object , js_name = PopupOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type PopupOptions;

    /// [`Popup`](https://leafletjs.com/reference.html#popup)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = DivOverlay)]
    pub type Popup;

    /// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &PopupOptions, layer: Option<&Layer>) -> Popup;

    /// [`L.popup`](/// [`L.popup`](https://leafletjs.com/reference.html#popup-l-popup))
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn newWithLatLng(lat_lng: &LatLng, options: &PopupOptions) -> Popup;
}

impl PopupOptions {
    object_constructor!();
    object_property_set!(pane, &str);
    object_property_set!(offset, Point);
    object_property_set!(min_width, minWidth, f64);
    object_property_set!(max_width, maxWidth, f64);
    object_property_set!(max_height, maxHeight, f64);
    object_property_set!(auto_pan, autoPan, bool);
    object_property_set!(auto_pan_padding_top_left, autoPanPaddingTopLeft, Point);
    object_property_set!(
        auto_pan_padding_bottom_right,
        autoPanPaddingBottomRight,
        Point
    );
    object_property_set!(auto_pan_padding, autoPanPadding, Point);
    object_property_set!(keep_in_view, keepInView, bool);
    object_property_set!(close_button, closeButton, bool);
    object_property_set!(auto_close, autoClose, bool);
    object_property_set!(close_on_escape_key, closeOnEscapeKey, bool);
    object_property_set!(close_on_click, closeOnClick, bool);
    object_property_set!(class_name, className, &str);
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self::new()
    }
}
