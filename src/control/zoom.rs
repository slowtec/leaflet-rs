use super::Control;
use crate::{object_constructor, object_property_set};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Control, js_namespace = ["L", "Control"])]
    pub type Zoom;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "zoom")]
    fn constructor_zoom(options: &ZoomOptions) -> Zoom;

    #[wasm_bindgen(extends = Object , js_name = ZoomOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[wasm_bindgen(extends = Control)]
    pub type ZoomOptions;
}

impl Zoom {
    /// Creates a new `Zoom` control.
    pub fn new(options: &ZoomOptions) -> Self {
        constructor_zoom(options)
    }
}

impl ZoomOptions {
    object_constructor!();

    // ZoomOptions
    object_property_set!(zoom_in_text, zoomInText, &str);
    object_property_set!(zoom_in_title, zoomInTitle, &str);
    object_property_set!(zoom_out_text, zoomOutText, &str);
    object_property_set!(zoom_out_title, zoomOutTitle, &str);

    // ControlOptions
    object_property_set!(position, position, &str);
}

impl Default for ZoomOptions {
    fn default() -> Self {
        ZoomOptions::new()
    }
}
