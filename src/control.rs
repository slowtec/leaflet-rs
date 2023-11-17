mod zoom;

use crate::{create_object_with_properties, Map};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub use zoom::{Zoom, ZoomOptions};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_namespace = L, js_name = Control)]
    pub type Control;

    #[wasm_bindgen(js_namespace = ["L"], js_name = "control")]
    fn constructor_control(options: &ControlOptions) -> Control;

    #[wasm_bindgen(method, js_name = "getPosition")]
    pub fn get_position(this: &Control) -> String;

    #[wasm_bindgen(method, js_name = "setPosition")]
    pub fn set_position(this: &Control, position: &str) -> Control;

    #[wasm_bindgen(method, js_name = "getContainer")]
    pub fn get_container(this: &Control) -> HtmlElement;

    #[wasm_bindgen(method, js_name = "addTo")]
    pub fn add_to(this: &Control, map: &Map) -> Control;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Control) -> Control;
}

create_object_with_properties!(
    (ControlOptions, ControlOptions),
    (position, position, String)
);

impl Default for ControlOptions {
    fn default() -> Self {
        ControlOptions::new()
    }
}

impl Control {
    /// Creates a new [Control] instance.
    pub fn new(options: &ControlOptions) -> Self {
        constructor_control(options)
    }

    /// The given closure is executed when the control
    /// is added to a map using [addTo](Control::addTo).
    pub fn on_add<C: Fn(&Map) -> HtmlElement + 'static>(&self, on_add: C) {
        let closure = Closure::wrap(Box::new(on_add) as Box<dyn Fn(&Map) -> HtmlElement>);
        js_sys::Reflect::set(self, &JsValue::from("onAdd"), closure.as_ref())
            .expect("Unable to set Control::onAdd()");
        closure.forget();
    }

    /// The given closure is executed when the control
    /// is removed from a map using [onRemove](Control::onRemove).
    pub fn on_remove<C: Fn(&Map) + 'static>(&self, on_remove: C) {
        let closure = Closure::wrap(Box::new(on_remove) as Box<dyn Fn(&Map)>);
        js_sys::Reflect::set(self, &JsValue::from("onRemove"), closure.as_ref())
            .expect("Unable to set Control::onRemove()");
        closure.forget();
    }
}
