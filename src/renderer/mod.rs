mod canvas;
mod svg;

use crate::Layer;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Base class for vector renderers.
    ///
    /// This is the base class that all vector renderers inherit from.
    /// Inherits from `Layer` and provides common functionality for rendering vector graphics.
    /// This is equivalent to `L.Renderer` in JavaScript Leaflet.
    ///
    /// Inheritance hierarchy: `Evented` -> `Layer` -> `Renderer`
    #[wasm_bindgen(extends = Layer)]
    #[derive(Debug, Clone)]
    pub type Renderer;

}

pub use canvas::{Canvas, CanvasOptions};
pub use svg::{Svg, SvgOptions};

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn svg_renderer_creation() {
        // Test default SVG renderer creation
        let svg = Svg::new();
        assert!(svg.is_instance_of::<Svg>());
    }

    #[wasm_bindgen_test]
    fn svg_renderer_with_options() {
        // Test SVG renderer with custom options
        let options = SvgOptions::default();
        options.set_padding(0.1);
        options.set_tolerance(5.0);

        let svg = Svg::with_options(&options);
        assert!(svg.is_instance_of::<Svg>());
    }

    #[wasm_bindgen_test]
    fn canvas_renderer_creation() {
        // Test default Canvas renderer creation
        let canvas = Canvas::new();
        assert!(canvas.is_instance_of::<Canvas>());
    }

    #[wasm_bindgen_test]
    fn canvas_renderer_with_options() {
        // Test Canvas renderer with custom options
        let options = CanvasOptions::default();
        options.set_padding(0.05);
        options.set_tolerance(3.0);

        let canvas = Canvas::with_options(&options);
        assert!(canvas.is_instance_of::<Canvas>());
    }
}
