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

    // Point

    #[derive(Debug)]
    pub type Point;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(x: u32, y: u32) -> Point;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Point) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Point) -> u32;

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
    pub fn setView(this: &Map, center: &LatLng, zoom: f64);

    #[wasm_bindgen(method, js_name = setView)]
    pub fn setViewWithOptions(this: &Map, center: &LatLng, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Map) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getCenter(this: &Map) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn getZoomScale(this: &Map, toZoom: f64, fromZoom: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn setZoom(this: &Map, zoom: f64);

    #[wasm_bindgen(method, js_name = setZoom)]
    pub fn setZoomWithOptions(this: &Map, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn zoomIn(this: &Map, delta: f64);

    #[wasm_bindgen(method, js_name = zoomIn)]
    pub fn zoomInWithOptions(this: &Map, delta: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn zoomOut(this: &Map, delta: f64);

    #[wasm_bindgen(method, js_name = zoomOut)]
    pub fn zoomOutWithOptions(this: &Map, delta: f64, options: &JsValue);

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundLatLng(this: &Map, latlng: &LatLng, zoom: f64);

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundLatLngWithOptions(this: &Map, latlng: &LatLng, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundPoint(this: &Map, offset: &Point, zoom: f64);

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundPointWithOptions(this: &Map, offset: &Point, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn fitBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method, js_name = fitBounds)]
    pub fn fitBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn fitWorld(this: &Map);

    #[wasm_bindgen(method, js_name = fitWorld)]
    pub fn fitWorldWithOptions(this: &Map, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn on(this: &Map, kind: &str, handler: &JsValue);

    #[wasm_bindgen(method)]
    pub fn panTo(this: &Map, latlng: &LatLng);

    #[wasm_bindgen(method, js_name = panTo)]
    pub fn panToWithOptions(this: &Map, latlng: &LatLng, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn panBy(this: &Map, point: &Point);

    #[wasm_bindgen(method, js_name = panBy)]
    pub fn panByWithOptions(this: &Map, point: &Point, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn flyTo(this: &Map, latlng: &LatLng, zoom: f64);

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn flyToWithOptions(this: &Map, latlng: &LatLng, zoom: f64, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn flyToBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method, js_name = flyToBounds)]
    pub fn flyToBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn setMaxBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method)]
    pub fn setMinZoom(this: &Map, zoom: f64);

    #[wasm_bindgen(method)]
    pub fn setMaxZoom(this: &Map, zoom: f64);

    #[wasm_bindgen(method)]
    pub fn panInsideBounds(this: &Map, bounds: &LatLngBounds);

    #[wasm_bindgen(method, js_name = panInsideBounds)]
    pub fn panInsideBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn panInside(this: &Map, latlng: &LatLng);

    #[wasm_bindgen(method, js_name = panInside)]
    pub fn panInsideWithOptions(this: &Map, latlng: &LatLng, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn invalidateSize(this: &Map, animate: bool);

    #[wasm_bindgen(method, js_name = invalidateSize)]
    pub fn invalidateSizeWithOptions(this: &Map, options: &JsValue);

    #[wasm_bindgen(method)]
    pub fn stop(this: &Map);

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

    // Polyline

    #[derive(Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlngs: Vec<JsValue>) -> Polyline;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlngs: Vec<JsValue>, options: &JsValue) -> Polyline;

    // Polygon

    #[derive(Debug)]
    #[wasm_bindgen(extends = Polyline)]
    pub type Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlngs: Vec<JsValue>) -> Polygon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlngs: Vec<JsValue>, options: &JsValue) -> Polygon;

    // Rectangle

    #[derive(Debug)]
    #[wasm_bindgen(extends = Polygon)]
    pub type Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(bounds: &LatLngBounds) -> Rectangle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(bounds: &LatLngBounds, options: &JsValue) -> Rectangle;

    // Circle

    #[derive(Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> Circle;

    // TileLayer

    #[derive(Debug)]
    pub type TileLayer;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(url_template: &str, options: &JsValue) -> TileLayer;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &TileLayer, map: &Map);

    // Control

    #[derive(Debug)]
    pub type Control;

    #[wasm_bindgen(js_namespace = L, static_method_of = Control)]
    pub fn extend(props: &JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn addTo(this: &Control, map: &Map);

}
