use crate::evented::{
    DragEvents, LeafletEventHandler, MouseEvents, MoveEvents, PopupEvents, TooltipEvents,
};
use crate::{
    create_object_with_properties, Evented, Handler, Icon, LatLng, Layer, LayerEvents, Point,
};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Marker
    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = Layer)]
    pub type Marker;

    // [`Marker`](https://leafletjs.com/reference.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat_lng: &LatLng) -> Marker;

    // [`Marker`](https://leafletjs.com/reference.html#marker-l-marker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(lat_lng: &LatLng, options: &MarkerOptions) -> Marker;

    /// ['setIcon'](https://leafletjs.com/reference.html#marker-seticon)
    #[wasm_bindgen(method, js_name = setIcon)]
    pub fn set_icon(this: &Marker, icon: &Icon);

    /// ['setIcon'](https://leafletjs.com/reference.html#marker-setopacity)
    #[wasm_bindgen(method, js_name = setOpacity)]
    pub fn set_opacity(this: &Marker, value: f64);

    /// ['getLatLng'](https://leafletjs.com/reference.html#marker-getlatlng)
    #[wasm_bindgen(method, js_name = getLatLng)]
    pub fn get_lat_lng(this: &Marker) -> LatLng;

    /// ['setLatLng'](https://leafletjs.com/reference.html#marker-setlatlng)
    #[wasm_bindgen(method, js_name = setLatLng)]
    pub fn set_lat_lng(this: &Marker, latlng: &LatLng);

    /// ['dragging'](https://leafletjs.com/reference.html#marker-dragging)
    #[wasm_bindgen(method, getter)]
    pub fn dragging(this: &Marker) -> Handler;

    #[wasm_bindgen(method)]
    pub fn update(this: &Marker) -> Marker;

    #[wasm_bindgen(method, js_name = setZIndexOffset)]
    pub fn set_z_index_offset(this: &Marker, offset: f64) -> Marker;
}

create_object_with_properties!(
    (MarkerOptions, MarkerOptions),
    // [`icon`](https://leafletjs.com/reference.html#marker-icon)
    (icon, icon, Icon),
    // ['keyboard'](https://leafletjs.com/reference.html#marker-keyboard)
    (keyboard, keyboard, bool),
    // ['title'](https://leafletjs.com/reference.html#marker-title)
    (title, title, String),
    // ['alt'](https://leafletjs.com/reference.html#marker-alt)
    (alt, alt, String),
    // ['zIndexOffset'](https://leafletjs.com/reference.html#marker-zindexoffset)
    (z_index_offset, zIndexOffset, f64),
    // ['opacity'](https://leafletjs.com/reference.html#marker-opacity)
    (opacity, opacity, f64),
    // ['riseOnHover'](https://leafletjs.com/reference.html#marker-riseonhover)
    (rise_on_hover, riseOnHover, bool),
    // ['riseOffset'](https://leafletjs.com/reference.html#marker-riseoffset)
    (rise_offset, riseOffset, f64),
    // ['pane'](https://leafletjs.com/reference.html#marker-pane)
    (pane, pane, String),
    // ['shadowPane'](https://leafletjs.com/reference.html#marker-shadowpane)
    (shadow_pane, shadowPane, String),
    // ['bubblingMouseEvents'](https://leafletjs.com/reference.html#marker-bubblingmouseevents)
    (bubbling_mouse_events, bubblingMouseEvents, bool),
    // Draggable marker options
    // ['draggable'](https://leafletjs.com/reference.html#marker-draggable)
    (draggable, draggable, bool),
    // ['autoPan'](https://leafletjs.com/reference.html#marker-autopan)
    (auto_pan, autoPan, bool),
    // ['autoPanPadding'](https://leafletjs.com/reference.html#marker-autopanpadding)
    (auto_pan_padding, autoPanPadding, Point),
    // ['autoPanSpeed'](https://leafletjs.com/reference.html#marker-autopanspeed)
    (auto_pan_speed, autoPanSpeed, f64),
    // Interactive layer
    // ['interactive'](https://leafletjs.com/reference.html#marker-interactive)
    (interactive, interactive, bool),
    // Layer
    // ['attribution'](https://leafletjs.com/reference.html#marker-attribution)
    (attribution, attribution, String)
);

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
