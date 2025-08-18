use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{create_object_with_properties, renderer::Renderer};

#[wasm_bindgen]
extern "C" {
    /// Canvas renderer for vector graphics.
    ///
    /// Allows vector layers to be displayed with Canvas. Inherits from `Renderer`.
    /// This is equivalent to `L.Canvas` in JavaScript Leaflet.
    ///
    /// Inheritance hierarchy: `Evented` -> `Layer` -> `Renderer` -> `Canvas`
    #[wasm_bindgen(extends = Renderer)]
    #[derive(Debug, Clone)]
    pub type Canvas;

    #[wasm_bindgen(js_namespace = L, js_name = canvas)]
    pub fn canvas_with_options(options: &CanvasOptions) -> Canvas;

    #[wasm_bindgen(js_namespace = L, js_name = canvas)]
    pub fn canvas() -> Canvas;
}

create_object_with_properties!(
    (CanvasOptions, CanvasOptions),
    // Renderer Options
    (padding, padding, f64),
    (tolerance, tolerance, f64),
    // Canvas specific options
    (stroke, stroke, bool),
    (color, color, String),
    (weight, weight, f64),
    (opacity, opacity, f64),
    (line_cap, lineCap, String),
    (line_join, lineJoin, String),
    (dash_array, dashArray, String),
    (dash_offset, dashOffset, String),
    (fill, fill, bool),
    (fill_color, fillColor, String),
    (fill_opacity, fillOpacity, f64),
    (fill_rule, fillRule, String),
    (interactive, interactive, bool),
    (bubbling_mouse_events, bubblingMouseEvents, bool),
    (class_name, className, String),
    (pane, pane, String)
);

impl Default for CanvasOptions {
    fn default() -> Self {
        CanvasOptions::new()
    }
}

impl Canvas {
    /// Creates a new Canvas renderer with default options.
    #[must_use]
    pub fn new() -> Self {
        canvas()
    }

    /// Creates a new Canvas renderer with the specified options.
    #[must_use]
    pub fn with_options(options: &CanvasOptions) -> Self {
        canvas_with_options(options)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
