use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = Event)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type Event;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Event;

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn eventType(this: &Event) -> String;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn setEventType(this: &Event, value: &str);

    #[wasm_bindgen(method, getter, js_name = target)]
    pub fn target(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = target)]
    pub fn setTarget(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = sourceTarget)]
    pub fn sourceTarget(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = sourceTarget)]
    pub fn setsourceTarget(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = propagatedFrom)]
    pub fn propagatedFrom(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = propagatedFrom)]
    pub fn setPropagatedFrom(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = layer)]
    pub fn layer(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = layer)]
    pub fn setLayer(this: &Event, value: &Object);
}
