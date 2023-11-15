use super::Control;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = Layers, extends = Control)]
    pub type LayerControl;

    #[wasm_bindgen(constructor)]
    pub fn new(base_layers: &JsValue) -> LayerControl;
}
