use crate::evented::LeafletEventHandler;
use crate::{Event, LatLng, LocationEvent, Point, Popup, PopupEvents, Tooltip, TooltipEvents};
use wasm_bindgen::prelude::*;

use super::Map;

#[wasm_bindgen]
extern "C" {
    /// Mouse Event
    #[wasm_bindgen (extends = Event, js_name = MouseEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type MouseEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MouseEvent;

    #[wasm_bindgen(method, getter, js_name = latlng)]
    pub fn lat_lng(this: &MouseEvent) -> LatLng;

    #[wasm_bindgen(method, setter, js_name = latlng)]
    pub fn set_lat_lng(this: &MouseEvent, value: &LatLng) -> MouseEvent;

    #[wasm_bindgen(method, getter, js_name = layerPoint)]
    pub fn layer_point(this: &MouseEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = layerPoint)]
    pub fn set_layer_point(this: &MouseEvent, value: &Point) -> MouseEvent;

    #[wasm_bindgen(method, getter, js_name = containerPoint)]
    pub fn container_point(this: &crate::map::events::MouseEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = containerPoint)]
    pub fn set_container_point(this: &MouseEvent, value: &Point) -> MouseEvent;

    #[wasm_bindgen(method, getter, js_name = originalEvent)]
    pub fn original_event(this: &MouseEvent) -> web_sys::MouseEvent;

    #[wasm_bindgen(method, setter, js_name = originalEvent)]
    pub fn set_original_event(
        this: &MouseEvent,
        value: &web_sys::MouseEvent,
    ) -> crate::map::events::MouseEvent;

    /// Error Event
    #[wasm_bindgen(extends = Event, js_name = ErrorEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type ErrorEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ErrorEvent;

    #[wasm_bindgen(method, getter, js_name = message)]
    pub fn message(this: &ErrorEvent) -> String;

    #[wasm_bindgen(method, setter, js_name = message)]
    pub fn set_message(this: &ErrorEvent, value: &str);

    #[wasm_bindgen(method, getter, js_name = code)]
    pub fn code(this: &ErrorEvent) -> i32;

    #[wasm_bindgen(method, setter, js_name = code)]
    pub fn set_code(this: &ErrorEvent, value: i32);

    /// Drag End Event
    #[wasm_bindgen (extends = Event, js_name = DragEndEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type DragEndEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> DragEndEvent;

    /// The distance in pixels the draggable element was moved by.
    #[wasm_bindgen(method, getter, js_name = distance)]
    pub fn distance(this: &DragEndEvent) -> f64;

    #[wasm_bindgen(method, setter, js_name = distance)]
    pub fn set_distance(this: &DragEndEvent, value: f64);

    /// Reset Event
    #[wasm_bindgen (extends = Event, js_name = ResetEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type ResetEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> ResetEvent;

    #[wasm_bindgen(method, getter, js_name = oldSize)]
    pub fn old_size(this: &ResetEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = oldSize)]
    pub fn set_old_size(this: &ResetEvent, value: &Point);

    #[wasm_bindgen(method, getter, js_name = newSize)]
    pub fn new_size(this: &ResetEvent) -> Point;

    #[wasm_bindgen(method, setter, js_name = newSize)]
    pub fn set_new_size(this: &ResetEvent, value: &Point);

    /// Popup Event
    #[wasm_bindgen(extends = Event, js_name = PopupEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type PopupEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> PopupEvent;

    #[wasm_bindgen(method, getter, js_name = popup)]
    pub fn popup(this: &PopupEvent) -> Popup;

    #[wasm_bindgen(method, setter, js_name = popup)]
    pub fn set_popup(this: &PopupEvent, value: &Popup);

    /// Tooltip Event
    #[wasm_bindgen(extends = Event, js_name = TooltipEvent)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type TooltipEvent;

    #[wasm_bindgen(constructor)]
    pub fn new() -> TooltipEvent;

    #[wasm_bindgen(method, getter, js_name = tooltip)]
    pub fn tooltip(this: &TooltipEvent) -> Popup;

    #[wasm_bindgen(method, setter, js_name = tooltip)]
    pub fn set_tooltip(this: &TooltipEvent, value: &Tooltip);
}

impl Map {
    pub fn on_zoom_levels_change(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomlevelschange", &closure.into_js_value());
    }

    pub fn on_resize(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("resize", &closure.into_js_value());
    }

    pub fn on_view_reset(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("viewreset", &closure.into_js_value());
    }

    pub fn on_load(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("load", &closure.into_js_value());
    }

    pub fn on_unload(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("unload", &closure.into_js_value());
    }

    pub fn on_zoom(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoom", &closure.into_js_value());
    }

    pub fn on_zoom_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomstart", &closure.into_js_value());
    }

    pub fn on_zoom_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("zoomend", &closure.into_js_value());
    }

    pub fn on_move(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("move", &closure.into_js_value());
    }

    pub fn on_move_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("movestart", &closure.into_js_value());
    }

    pub fn on_move_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("moveend", &closure.into_js_value());
    }

    pub fn on_location_found(&self, callback: Box<dyn Fn(LocationEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("locationfound", &closure.into_js_value());
    }

    pub fn on_location_error(&self, callback: Box<dyn Fn(ErrorEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("locationerror", &closure.into_js_value());
    }

    pub fn on_popup_open(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupopen", &closure.into_js_value());
    }

    pub fn on_popup_close(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupclose", &closure.into_js_value());
    }

    pub fn on_mouse_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("click", &closure.into_js_value());
    }

    pub fn on_mouse_double_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("dblclick", &closure.into_js_value());
    }

    pub fn on_mouse_context_menu(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("contextmenu", &closure.into_js_value());
    }

    pub fn on_mouse_move(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mousemove", &closure.into_js_value());
    }

    pub fn on_mouse_over(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseover", &closure.into_js_value());
    }

    pub fn on_mouse_out(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseout", &closure.into_js_value());
    }

    pub fn on_mouse_down(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mousedown", &closure.into_js_value());
    }

    pub fn on_mouse_up(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseup", &closure.into_js_value());
    }
}

impl LeafletEventHandler for Map {
    fn on(&self, event: &str, callback: &JsValue) {
        self.obj.on(event, callback);
    }
}

impl TooltipEvents for Map {}
impl PopupEvents for Map {}
