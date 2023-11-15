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
    pub fn event_type(this: &Event) -> String;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn set_event_type(this: &Event, value: &str);

    #[wasm_bindgen(method, getter, js_name = target)]
    pub fn target(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = target)]
    pub fn set_target(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = sourceTarget)]
    pub fn source_target(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = sourceTarget)]
    pub fn set_source_target(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = propagatedFrom)]
    pub fn propagated_from(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = propagatedFrom)]
    pub fn set_propagated_from(this: &Event, value: &Object);

    #[wasm_bindgen(method, getter, js_name = layer)]
    pub fn layer(this: &Event) -> Object;

    #[wasm_bindgen(method, setter, js_name = layer)]
    pub fn set_layer(this: &Event, value: &Object);
}
