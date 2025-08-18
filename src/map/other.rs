use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::Map;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(method, js_name = addHandler)]
    pub fn add_handler(this: &Map, name: &str, handler_class: &Closure<dyn Fn()>) -> Map;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &Map) -> Map;

    #[wasm_bindgen(method, js_name = createPane)]
    pub fn create_pane(this: &Map, name: &str, container: &HtmlElement) -> HtmlElement;

    #[wasm_bindgen(method, js_name = createPane)]
    pub fn create_pane_by_name(this: &Map, name: &str) -> HtmlElement;

    #[wasm_bindgen(method, js_name = getPane)]
    pub fn get_pane(this: &Map, pane: &HtmlElement) -> HtmlElement;

    #[wasm_bindgen(method, js_name = getPane)]
    pub fn get_pane_by_name(this: &Map, name: &str) -> HtmlElement;

    #[wasm_bindgen(method, js_name = getPanes)]
    pub fn get_panes(this: &Map) -> Object;

    #[wasm_bindgen(method, js_name = getContainer)]
    pub fn get_container(this: &Map) -> HtmlElement;

    #[wasm_bindgen(method, js_name = whenReady)]
    pub fn when_ready(this: &Map, callback: &Closure<dyn Fn(Map, Object)>) -> Map;

    #[wasm_bindgen(method, js_name = whenReady)]
    pub fn when_ready_with_context(
        this: &Map,
        callback: &Closure<dyn Fn(JsValue)>,
        context: &JsValue,
    ) -> Map;
}
