mod control;
mod div_icon;
mod div_overlay;
mod event;
mod evented;
mod grid_layer;
mod handler;
mod icon;
mod lat_lng;
mod layer;
mod layer_control;
mod layer_group;
mod map;
mod marker;
mod popup;
mod raster;
mod shapes;
mod tooltip;

use js_sys::Array;
use wasm_bindgen::prelude::*;

pub use control::{Control, ControlOptions, Zoom, ZoomOptions};
pub use div_icon::{DivIcon, DivIconOptions};
pub use div_overlay::DivOverlay;
pub use event::Event;
pub use evented::{
    DragEvents, Evented, EventedHandle, LayerEvents, MouseEvents, MoveEvents, PopupEvents,
    TooltipEvents,
};
pub use grid_layer::{GridLayer, GridLayerOptions};
pub use handler::Handler;
pub use icon::{Icon, IconOptions};
pub use lat_lng::LatLng;
pub use layer::Layer;
pub use layer_group::LayerGroup;
pub use map::{
    DragEndEvent, ErrorEvent, LocateOptions, LocationEvent, Map, MapOptions, MouseEvent,
    PopupEvent, TooltipEvent,
};
pub use marker::{Marker, MarkerOptions};
pub use popup::{Popup, PopupOptions};
pub use raster::{
    ImageOverlay, ImageOverlayOptions, TileLayer, TileLayerOptions, TileLayerWms,
    TileLayerWmsOptions, VideoOverlay, VideoOverlayOptions,
};
pub use shapes::{
    Circle, CircleMarker, CircleOptions, Path, PathOptions, Polygon, Polyline, PolylineOptions,
    Rectangle,
};
pub use tooltip::{Tooltip, TooltipOptions};

#[macro_export]
macro_rules! object_property_set {
    ($a:ident, $b:ty) => {
        pub fn $a(&mut self, val: $b) {
            let _ = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($a)),
                &wasm_bindgen::JsValue::from(val),
            );
        }
    };
    ($a:ident, $b:ident, $c:ty) => {
        pub fn $a(&mut self, val: $c) {
            let _ = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from(val),
            );
        }
    };
}

#[macro_export]
macro_rules! object_property_set_with {
    ($a:ident, $b:ident, $c:expr) => {
        pub fn $a(&mut self) {
            let _ = js_sys::Reflect::set(
                self.as_ref(),
                &wasm_bindgen::JsValue::from(stringify!($b)),
                &wasm_bindgen::JsValue::from($c),
            );
        }
    };
}

#[macro_export]
macro_rules! object_constructor {
    () => {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            #[allow(unused_mut)]
            let mut r = JsCast::unchecked_into(Object::new());
            r
        }
    };
}

#[wasm_bindgen]
extern "C" {
    // CRS
    #[derive(Debug)]
    pub type Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Simple")]
    pub fn new_simple() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Earth")]
    pub fn new_earth() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG3395")]
    pub fn new_epsg_3395() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG3857")]
    pub fn new_epsg_3857() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "EPSG4326")]
    pub fn new_epsg_4326() -> Crs;

    #[wasm_bindgen(constructor, js_namespace = ["L", "CRS"], js_name = "Base")]
    pub fn new_base() -> Crs;

    #[wasm_bindgen(method, js_name = "latLngToPoint")]
    pub fn latLngToPoint(this: &Crs, latlng: LatLng, zoom: f32) -> Point;

    // Point
    #[derive(Debug)]
    pub type Point;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(x: f64, y: f64) -> Point;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Point) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Point) -> f64;

    #[wasm_bindgen(method)]
    pub fn add(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn subtract(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn multiplyBy(this: &Point, scalar: f64) -> Point;

    #[wasm_bindgen(method)]
    pub fn divideBy(this: &Point, scalar: f64) -> Point;

    #[wasm_bindgen(method)]
    pub fn scaleBy(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn unscaleByTo(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn round(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn floor(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn trunc(this: &Point) -> bool;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Point, other: &Point) -> bool;

    #[wasm_bindgen(method)]
    pub fn contains(this: &Point, other: &Point) -> f64;

    #[wasm_bindgen(method)]
    pub fn distanceTo(this: &Point, other: &Point) -> f64;

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

    // FeatureGroup

    /// [`FeatureGroup`](https://leafletjs.com/reference-1.7.1.html#featuregroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = LayerGroup)]
    pub type FeatureGroup;

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#featuregroup-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &FeatureGroup, style: &JsValue);

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &FeatureGroup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#featuregroup-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &FeatureGroup);

    /// [`getBounds`](https://leafletjs.com/reference-1.7.1.html#featuregroup-getbounds)
    #[wasm_bindgen(method)]
    pub fn getBounds(this: &FeatureGroup) -> LatLngBounds;

    // GeoJSON

    /// [`GeoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type GeoJSON;

    /// [`L.geoJSON`](https://leafletjs.com/reference-1.7.1.html#geojson-l-geojson)
    #[wasm_bindgen(js_namespace = L)]
    pub fn geoJSON(geojson: &JsValue, options: &JsValue) -> GeoJSON;

    /// [`addData`](https://leafletjs.com/reference-1.7.1.html#geojson-adddata)
    #[wasm_bindgen(method)]
    pub fn addData(this: &GeoJSON, data: &JsValue);

    /// [`resetStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-resetstyle)
    #[wasm_bindgen(method)]
    pub fn resetStyle(this: &GeoJSON, layer: Option<&Layer>);

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#geojson-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &GeoJSON, style: &JsValue);
}

#[allow(clippy::from_over_into)]
impl Into<LatLngBounds> for (LatLng, LatLng) {
    fn into(self) -> LatLngBounds {
        LatLngBounds::new(&self.0, &self.1)
    }
}

pub fn to_lat_lng_array<T: Into<LatLng> + Clone>(lat_lngs: &[T]) -> Array {
    let array = Array::new();
    for lat_lng in lat_lngs.iter().cloned() {
        array.push(&lat_lng.into());
    }
    array
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Point {
        Point::new(x as f64, y as f64)
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point::new(x, y)
    }
}
