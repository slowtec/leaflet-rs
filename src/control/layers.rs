use super::Control;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Control, js_namespace = ["L", "Control"])]
    pub type LayersControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "layers")]
    fn constructor_layers(options: &Object) -> LayersControl;
}

impl LayersControl {
    #[must_use]
    pub fn new(options: &Object) -> Self {
        constructor_layers(options)
    }
}
