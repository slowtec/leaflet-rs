use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{create_object_with_properties, renderer::Renderer};

#[wasm_bindgen]
extern "C" {
    /// SVG renderer for vector graphics.
    ///
    /// Allows vector layers to be displayed with SVG. Inherits from `Renderer`.
    /// This is equivalent to `L.SVG` in JavaScript Leaflet.
    ///
    /// Inheritance hierarchy: `Evented` -> `Layer` -> `Renderer` -> `Svg`
    #[wasm_bindgen(extends = Renderer)]
    #[derive(Debug, Clone)]
    pub type Svg;

    #[wasm_bindgen(js_namespace = L, js_name = svg)]
    pub fn svg_with_options(options: &SvgOptions) -> Svg;

    #[wasm_bindgen(js_namespace = L, js_name = svg)]
    pub fn svg() -> Svg;
}

create_object_with_properties!(
    (SvgOptions, SvgOptions),
    // Renderer Options
    (padding, padding, f64),
    (tolerance, tolerance, f64),
    // SVG specific options
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

impl Default for SvgOptions {
    fn default() -> Self {
        SvgOptions::new()
    }
}

impl Svg {
    /// Creates a new SVG renderer with default options.
    #[must_use]
    pub fn new() -> Self {
        svg()
    }

    /// Creates a new SVG renderer with the specified options.
    #[must_use]
    pub fn with_options(options: &SvgOptions) -> Self {
        svg_with_options(options)
    }
}

impl Default for Svg {
    fn default() -> Self {
        Self::new()
    }
}
