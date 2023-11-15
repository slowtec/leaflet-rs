mod zoom;

use crate::{object_constructor, object_property_set, Map};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub use zoom::{Zoom, ZoomOptions};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object , js_name = ControlOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ControlOptions;

    #[derive(Clone, Debug)]
    #[wasm_bindgen(js_namespace = L, js_name = Control)]
    pub type Control;

    #[wasm_bindgen(js_namespace = ["L"], js_name = "control")]
    fn constructor_control(options: &ControlOptions) -> Control;

    #[wasm_bindgen(method)]
    pub fn getPosition(this: &Control) -> String;

    #[wasm_bindgen(method)]
    pub fn setPosition(this: &Control, position: &str) -> Control;

    #[wasm_bindgen(method)]
    pub fn getContainer(this: &Control) -> HtmlElement;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Control, map: &Map) -> Control;

    #[wasm_bindgen(method)]
    pub fn remove(this: &Control) -> Control;
}

impl ControlOptions {
    object_constructor!();

    // ControlOptions
    object_property_set!(position, position, &str);
}

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
        js_sys::Reflect::set(&self, &JsValue::from("onAdd"), closure.as_ref())
            .expect("Unable to set Control::onAdd()");
        closure.forget();
    }

    /// The given closure is executed when the control
    /// is removed from a map using [onRemove](Control::onRemove).
    pub fn on_remove<C: Fn(&Map) + 'static>(&self, on_remove: C) {
        let closure = Closure::wrap(Box::new(on_remove) as Box<dyn Fn(&Map)>);
        js_sys::Reflect::set(&self, &JsValue::from("onRemove"), closure.as_ref())
            .expect("Unable to set Control::onRemove()");
        closure.forget();
    }
}
