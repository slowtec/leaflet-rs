use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{object_constructor, object_property_set, Layer};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object, js_name = PathOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type PathOptions;

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

impl PathOptions {
    object_constructor!();
    object_property_set!(stroke, bool);
    object_property_set!(color, &str);
    object_property_set!(weight, f64);
    object_property_set!(interactive, bool);
    object_property_set!(opacity, f64);
    object_property_set!(line_cap, lineCap, &str);
    object_property_set!(line_join, lineJoin, &str);
    object_property_set!(dash_array, dashArray, &str);
    object_property_set!(dash_offset, dashOffset, &str);
    object_property_set!(fill, bool);
    object_property_set!(fill_color, fillColor, &str);
    object_property_set!(fill_opacity, fillOpacity, f64);
    object_property_set!(fill_rule, fillRule, &str);
    object_property_set!(bubbling_mouse_events, bubblingMouseEvents, bool);
    object_property_set!(renderer, &JsValue);
    object_property_set!(class_name, className, &str);
}

impl Default for PathOptions {
    fn default() -> Self {
        PathOptions::new()
    }
}
