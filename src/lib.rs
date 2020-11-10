use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {

    // mapboxGl
    #[derive(Debug)]
    pub type mapboxGL;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &JsValue) -> mapboxGL;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &mapboxGL, map: &Map);

    // Evented

    #[derive(Debug, Clone)]
    pub type Evented;

    #[wasm_bindgen(method)]
    pub fn on(this: &Evented, kind: &str, handler: &JsValue);

    // Icon

    #[derive(Debug)]
    pub type Icon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &JsValue) -> Icon;

    // LatLng

    #[derive(Debug)]
    pub type LatLng;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat: f64, lng: f64) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LatLng) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LatLng) -> f64;

    #[wasm_bindgen(method)]
    pub fn distanceTo(this: &LatLng, otherLatLng: &LatLng) -> f64;

    // LatLngBounds

    #[derive(Debug)]
    pub type LatLngBounds;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(corner1: &LatLng, corner2: &LatLng) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getNorthEast(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getSouthWest(this: &LatLngBounds) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn contains(this: &LatLngBounds, latlng: &LatLng) -> bool;

    // Layer

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Evented)]
    pub type Layer;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Layer, map: &Map);

    #[wasm_bindgen(method)]
    pub fn remove(this: &Layer);

    // LayerGroup

    #[derive(Debug)]
    pub type LayerGroup;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> LayerGroup;

    #[wasm_bindgen(method)]
    pub fn addLayer(this: &LayerGroup, layer: &Layer);

    #[wasm_bindgen(method)]
    pub fn addTo(this: &LayerGroup, map: &Map);

    #[wasm_bindgen(method)]
    pub fn clearLayers(this: &LayerGroup);

    // Map

    #[derive(Debug)]
    pub type Map;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(id: &str, options: &JsValue) -> Map;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_element(el: &HtmlElement, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Map) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getCenter(this: &Map) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn getZoomScale(this: &Map, toZoom: f64, fromZoom: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn fitBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method)]
    pub fn on(this: &Map, kind: &str, handler: &JsValue);

    #[wasm_bindgen(method)]
    pub fn flyTo(this: &Map, latlng: &LatLng, zoom: f64);

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn flyToWithOptions(this: &Map, latlng: &LatLng, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn flyToBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method, js_name = flyToBounds)]
    pub fn flyToBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue);

    // Marker

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Marker;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng, options: &JsValue) -> Marker;

    #[wasm_bindgen(method)]
    pub fn setIcon(this: &Marker, icon: &Icon);

    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Marker, latlng: &LatLng);

    #[wasm_bindgen(method)]
    pub fn on(this: &Marker, event_name: &str, handler: &JsValue);

    // MouseEvent

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Event)]
    pub type MouseEvent;

    #[wasm_bindgen(method, getter)]
    pub fn latlng(this: &MouseEvent) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn originalEvent(this: &MouseEvent) -> web_sys::Event;

    // Event

    #[derive(Debug, Clone)]
    pub type Event;

    #[wasm_bindgen(method, getter)]
    pub fn target(this: &Event) -> Object;

    #[wasm_bindgen(method, getter)]
    pub fn sourceTarget(this: &Event) -> Object;

    // Rectangle

    #[derive(Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(bounds: &LatLngBounds) -> Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(bounds: &LatLngBounds, options: &JsValue) -> Rectangle;

    // TileLayer

    #[derive(Debug)]
    pub type TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(url_template: &str, options: &JsValue) -> TileLayer;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &TileLayer, map: &Map);

}
