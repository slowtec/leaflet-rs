use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = DivOverlay)]
    #[derive(Debug, Clone)]
    pub type Handler;

    /// Create a new handler
    #[wasm_bindgen(method)]
    pub fn enable(this: &Handler) -> Handler;

    /// Remove the handler
    #[wasm_bindgen(method)]
    pub fn disable(this: &Handler) -> Handler;

    /// Check if the handler is enabled
    #[wasm_bindgen(method)]
    pub fn enabled(this: &Handler) -> bool;
}
