use std::borrow::Cow;

use crate::map::{DragEndEvent, TooltipEvent};
use crate::{
    Circle, CircleMarker, Event, Layer, Map, Marker, MouseEvent, Path, Polygon, Polyline,
    PopupEvent,
};
use js_sys::Object;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone, PartialEq)]
    pub type Evented;

    /// Creates a new Evented object.
    ///
    /// [`on`](https://leafletjs.com/reference.html#evented-on)
    #[wasm_bindgen(method, js_name = on)]
    pub fn on(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    /// Removes an event listener.
    ///
    /// [`off`](https://leafletjs.com/reference.html#evented-off)
    #[wasm_bindgen(method)]
    pub fn off(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    #[wasm_bindgen(method, js_name = off)]
    pub fn off_by_name(this: &Evented, kind: &str) -> Evented;

    /// Removes all event listeners.
    ///
    /// [`off`](https://leafletjs.com/reference.html#evented-off)
    #[wasm_bindgen(method, js_name = off)]
    pub fn off_all(this: &Evented) -> Evented;

    /// Emits an event.
    ///
    /// [`fire`](https://leafletjs.com/reference.html#evented-fire)
    #[wasm_bindgen(method)]
    pub fn fire(this: &Evented, kind: &str, data: &Object, propagate: Option<bool>) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`listens`](https://leafletjs.com/reference.html#evented-listens)
    #[wasm_bindgen(method)]
    pub fn listens(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;

    /// Returns true if the event has listeners.
    ///
    /// [`once`](https://leafletjs.com/reference.html#evented-once)
    #[wasm_bindgen(method)]
    pub fn once(this: &Evented, kind: &str, handler: &JsValue) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`addEventParent`](https://leafletjs.com/reference.html#evented-addeventparent)
    #[wasm_bindgen(method, js_name = "addEventParent")]
    pub fn add_event_parent(this: &Evented, other: &Evented) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// ['removeEventParent'](https://leafletjs.com/reference.html#evented-removeeventparent)
    #[wasm_bindgen(method, js_name = "removeEventParent")]
    pub fn remove_event_parent(this: &Evented, other: &Evented) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`addEventListener`](https://leafletjs.com/reference.html#evented-addeventlistener)
    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`removeEventListener`](https://leafletjs.com/reference.html#evented-removeeventlistener)
    #[wasm_bindgen(method, js_name = "removeEventListener")]
    pub fn remove_event_listener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Clears all event listeners.
    ///
    /// [`clearAllEventListeners`](https://leafletjs.com/reference.html#evented-cleareventlisteners)
    #[wasm_bindgen(method, js_name = "clearAllEventListeners")]
    pub fn clear_all_event_listeners(this: &Evented) -> Evented;

    /// Adds a one time event listener.
    ///
    /// [`addOneTimeEventListener`](https://leafletjs.com/reference.html#evented-addonetimeeventlistener)
    #[wasm_bindgen(method, js_name = "addOneTimeEventListener")]
    pub fn add_one_time_event_listener(
        this: &Evented,
        kind: &str,
        handler: &Closure<dyn Fn(JsValue)>,
    ) -> Evented;

    /// Fires an event.
    ///
    /// [`fireEvent`](https://leafletjs.com/reference.html#evented-fireevent)
    #[wasm_bindgen(method, js_name = "fireEvent")]
    pub fn fire_event(
        this: &Evented,
        kind: &str,
        data: &Object,
        propagate: Option<bool>,
    ) -> Evented;

    /// Returns true if the event has listeners.
    ///
    /// [`hasEventListeners`](https://leafletjs.com/reference.html#evented-haseventlisteners)
    #[wasm_bindgen(method, js_name = "hasEventListeners")]
    pub fn has_event_listeners(this: &Evented, kind: &str, propagate: Option<bool>) -> bool;
}

pub trait FromLeafletEvent: FromWasmAbi {
    type EventType;
    fn from_leaflet_event(event: Self::EventType) -> Self;
}

impl FromLeafletEvent for Event {
    type EventType = Event;
    fn from_leaflet_event(event: Self::EventType) -> Self {
        event
    }
}

impl FromLeafletEvent for MouseEvent {
    type EventType = Event;
    fn from_leaflet_event(event: Self::EventType) -> Self {
        event.unchecked_into()
    }
}

impl FromLeafletEvent for DragEndEvent {
    type EventType = Event;
    fn from_leaflet_event(event: Self::EventType) -> Self {
        event.unchecked_into()
    }
}

impl FromLeafletEvent for TooltipEvent {
    type EventType = Event;
    fn from_leaflet_event(event: Self::EventType) -> Self {
        event.unchecked_into()
    }
}

pub struct EventedHandle<T: FromLeafletEvent> {
    target: Evented,
    event_type: Cow<'static, str>,
    callback: Closure<dyn FnMut(T)>,
}

impl<T: FromLeafletEvent> EventedHandle<T> {
    #[must_use]
    pub fn callback(&self) -> &Closure<dyn FnMut(T)> {
        &self.callback
    }
}

impl<T: FromLeafletEvent> Drop for EventedHandle<T> {
    fn drop(&mut self) {
        self.target
            .off(&self.event_type, self.callback.as_ref().unchecked_ref());
    }
}

impl Evented {
    pub fn on_leaflet_event<E, F, S, T>(target: &T, event: S, callback: F) -> EventedHandle<E>
    where
        T: EventedTarget,
        E: FromLeafletEvent + 'static,
        F: FnMut(E) + 'static,
        S: Into<Cow<'static, str>>,
    {
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(E)>);
        let event_type = event.into();
        let target = target.as_evented();
        target.on(&event_type, callback.as_ref().unchecked_ref());
        EventedHandle {
            target,
            event_type,
            callback,
        }
    }
}

pub trait EventedTarget {
    fn as_evented(&self) -> Evented;
}

impl EventedTarget for Layer {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Layer {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Evented {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Map {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for Marker {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Marker {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Path {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Polygon {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Polyline {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &Circle {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

impl EventedTarget for &CircleMarker {
    fn as_evented(&self) -> Evented {
        self.unchecked_ref::<Evented>().clone()
    }
}

pub trait LeafletEventHandler {
    fn on(&self, event: &str, callback: &JsValue);
}

pub trait MoveEvents
where
    Self: LeafletEventHandler,
{
    fn on_move(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("move", &closure.into_js_value());
    }
}

pub trait MouseEvents
where
    Self: LeafletEventHandler,
{
    fn on_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("click", &closure.into_js_value());
    }

    fn on_double_click(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("dblclick", &closure.into_js_value());
    }

    fn on_mouse_down(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mousedown", &closure.into_js_value());
    }

    fn on_mouse_up(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseup", &closure.into_js_value());
    }

    fn on_mouse_over(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseover", &closure.into_js_value());
    }

    fn on_mouse_out(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("mouseout", &closure.into_js_value());
    }

    fn on_context_menu(&self, callback: Box<dyn Fn(MouseEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("contextmenu", &closure.into_js_value());
    }
}

pub trait DragEvents
where
    Self: LeafletEventHandler,
{
    fn on_drag_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("dragstart", &closure.into_js_value());
    }

    fn on_move_start(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("movestart", &closure.into_js_value());
    }

    fn on_drag(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("drag", &closure.into_js_value());
    }

    fn on_drag_end(&self, callback: Box<dyn Fn(DragEndEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("dragend", &closure.into_js_value());
    }

    fn on_move_end(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("moveend", &closure.into_js_value());
    }
}

pub trait LayerEvents
where
    Self: LeafletEventHandler,
{
    fn on_add(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("add", &closure.into_js_value());
    }

    fn on_remove(&self, callback: Box<dyn Fn(Event)>) {
        let closure = Closure::wrap(callback);
        self.on("remove", &closure.into_js_value());
    }
}

pub trait PopupEvents
where
    Self: LeafletEventHandler,
{
    fn on_popup_open(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupopen", &closure.into_js_value());
    }

    fn on_popup_close(&self, callback: Box<dyn Fn(PopupEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("popupclose", &closure.into_js_value());
    }
}

pub trait TooltipEvents
where
    Self: LeafletEventHandler,
{
    fn on_tooltip_open(&self, callback: Box<dyn Fn(TooltipEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("tooltipopen", &closure.into_js_value());
    }

    fn on_tooltip_close(&self, callback: Box<dyn Fn(TooltipEvent)>) {
        let closure = Closure::wrap(callback);
        self.on("tooltipclose", &closure.into_js_value());
    }
}
