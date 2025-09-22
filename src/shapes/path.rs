use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{Layer, LayerOptions, create_object_with_properties};

#[wasm_bindgen]
extern "C" {
    /// [`Path`](https://leafletjs.com/reference.html#path)
    #[wasm_bindgen(extends = Layer)]
    #[derive(Debug, Clone)]
    pub type Path;

    /// [`redraw`](https://leafletjs.com/reference.html#path-redraw)
    #[wasm_bindgen(method)]
    pub fn redraw(this: &Path);

    /// [`setStyle`](https://leafletjs.com/reference.html#path-setstyle)
    #[wasm_bindgen(method, js_name = setStyle)]
    pub fn set_style(this: &Path, path_options: &PathOptions);

    /// [`bringToFront`](https://leafletjs.com/reference.html#path-bringtofront)
    #[wasm_bindgen(method, js_name = bringToFront)]
    pub fn bring_to_front(this: &Path);

    /// [`bringToBack`](https://leafletjs.com/reference.html#path-bringtoback)
    #[wasm_bindgen(method, js_name = bringToBack)]
    pub fn bring_to_back(this: &Path);
}

create_object_with_properties!(
    (PathOptions, PathOptions, LayerOptions),
    (stroke, stroke, bool),
    (color, color, String),
    (weight, weight, f64),
    (interactive, interactive, bool),
    (opacity, opacity, f64),
    (line_cap, lineCap, String),
    (line_join, lineJoin, String),
    (dash_array, dashArray, String),
    (dash_offset, dashOffset, String),
    (fill, fill, bool),
    (fill_color, fillColor, String),
    (fill_opacity, fillOpacity, f64),
    (fill_rule, fillRule, String),
    (bubbling_mouse_events, bubblingMouseEvents, bool),
    (renderer, renderer, JsValue),
    (class_name, className, String)
);

impl Default for PathOptions {
    fn default() -> Self {
        PathOptions::new()
    }
}
