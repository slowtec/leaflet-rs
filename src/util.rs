//! L.Util bindings.

use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["L", "Util"], js_name = "getParamString")]
    fn get_param_string(params: Object) -> String;

    #[wasm_bindgen(js_namespace = ["L", "Util"], js_name = "getParamString")]
    fn get_param_string_url(params: Object, url: &str) -> String;

    #[wasm_bindgen(js_namespace = ["L", "Util"], js_name = "getParamString")]
    fn get_param_string_url_uppercase(params: Object, url: &str, uppercase: bool) -> String;
}

/// The `L.Util` namespace.
pub struct Util;

impl Util {
    /// [`getParamString`](https://leafletjs.com/reference.html#util-getparamstring).
    pub fn get_param_string(params: Object) -> String {
        get_param_string(params)
    }
    /// [`getParamString`](https://leafletjs.com/reference.html#util-getparamstring).
    pub fn get_param_string_url(params: Object, url: &str) -> String {
        get_param_string_url(params, url)
    }
    /// [`getParamString`](https://leafletjs.com/reference.html#util-getparamstring).
    pub fn get_param_string_url_uppercase(params: Object, url: &str, uppercase: bool) -> String {
        get_param_string_url_uppercase(params, url, uppercase)
    }
}
