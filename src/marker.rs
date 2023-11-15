use crate::evented::{
    DragEvents, LeafletEventHandler, MouseEvents, MoveEvents, PopupEvents, TooltipEvents,
};
use crate::{
    object_constructor, object_property_set, Evented, Handler, Icon, LatLng, Layer, LayerEvents,
    Point,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = MarkerOptions)]
    #[derive(Debug, Clone)]
    pub type MarkerOptions;

    // Marker
    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = Layer)]
    pub type Marker;

    // [`Marker`](https://leafletjs.com/reference.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat_lng: &LatLng) -> Marker;

    // [`Marker`](https://leafletjs.com/reference.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn newWithOptions(lat_lng: &LatLng, options: &MarkerOptions) -> Marker;

    /// ['setIcon'](https://leafletjs.com/reference.html#marker-seticon)
    #[wasm_bindgen(method)]
    pub fn setIcon(this: &Marker, icon: &Icon);

    /// ['setIcon'](https://leafletjs.com/reference.html#marker-setopacity)
    #[wasm_bindgen(method)]
    pub fn setOpacity(this: &Marker, value: f64);

    /// ['getLatLng'](https://leafletjs.com/reference.html#marker-getlatlng)
    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Marker) -> LatLng;

    /// ['setLatLng'](https://leafletjs.com/reference.html#marker-setlatlng)
    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Marker, latlng: &LatLng);

    /// ['dragging'](https://leafletjs.com/reference.html#marker-dragging)
    #[wasm_bindgen(method, getter)]
    pub fn dragging(this: &Marker) -> Handler;

    #[wasm_bindgen(method)]
    pub fn update(this: &Marker) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setZIndexOffset(this: &Marker, offset: f64) -> Marker;
}

impl MarkerOptions {
    object_constructor!();
    // [`icon`](https://leafletjs.com/reference.html#marker-icon)
    object_property_set!(icon, Icon);
    // ['keyboard'](https://leafletjs.com/reference.html#marker-keyboard)
    object_property_set!(keyboard, bool);
    // ['title'](https://leafletjs.com/reference.html#marker-title)
    object_property_set!(title, &str);
    // ['alt'](https://leafletjs.com/reference.html#marker-alt)
    object_property_set!(alt, &str);
    // ['zIndexOffset'](https://leafletjs.com/reference.html#marker-zindexoffset)
    object_property_set!(z_index_offset, zIndexOffset, f64);
    // ['opacity'](https://leafletjs.com/reference.html#marker-opacity)
    object_property_set!(opacity, f64);
    // ['riseOnHover'](https://leafletjs.com/reference.html#marker-riseonhover)
    object_property_set!(rise_on_hover, riseOnHover, bool);
    // ['riseOffset'](https://leafletjs.com/reference.html#marker-riseoffset)
    object_property_set!(rise_offset, riseOffset, f64);
    // ['pane'](https://leafletjs.com/reference.html#marker-pane)
    object_property_set!(pane, &str);
    // ['shadowPane'](https://leafletjs.com/reference.html#marker-shadowpane)
    object_property_set!(shadow_pane, shadowPane, &str);
    // ['bubblingMouseEvents'](https://leafletjs.com/reference.html#marker-bubblingmouseevents)
    object_property_set!(bubbling_mouse_events, bubblingMouseEvents, bool);
    // Draggable marker options
    // ['draggable'](https://leafletjs.com/reference.html#marker-draggable)
    object_property_set!(draggable, bool);
    // ['autoPan'](https://leafletjs.com/reference.html#marker-autopan)
    object_property_set!(auto_pan, autoPan, bool);
    // ['autoPanPadding'](https://leafletjs.com/reference.html#marker-autopanpadding)
    object_property_set!(auto_pan_padding, autoPanPadding, Point);
    // ['autoPanSpeed'](https://leafletjs.com/reference.html#marker-autopanspeed)
    object_property_set!(auto_pan_speed, autoPanSpeed, f64);
    // Interactive layer
    // ['interactive'](https://leafletjs.com/reference.html#marker-interactive)
    object_property_set!(interactive, bool);
    // Layer
    // ['attribution'](https://leafletjs.com/reference.html#marker-attribution)
    object_property_set!(attribution, &str);
}

impl Default for MarkerOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl LeafletEventHandler for Marker {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl MoveEvents for Marker {}
impl MouseEvents for Marker {}
impl DragEvents for Marker {}
impl LayerEvents for Marker {}
impl PopupEvents for Marker {}
impl TooltipEvents for Marker {}
