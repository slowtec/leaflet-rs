mod control;
mod crs;
mod div_icon;
mod div_overlay;
mod event;
mod evented;
mod feature_group;
mod geo_json;
mod grid_layer;
mod handler;
mod icon;
mod lat_lng;
mod lat_lng_bounds;
mod layer;
mod layer_group;
mod map;
mod marker;
mod point;
mod popup;
mod raster;
mod shapes;
mod tooltip;
mod util;

use js_sys::Array;
use paste::paste;

pub use control::{Control, ControlOptions, LayersControl, Zoom, ZoomOptions};
pub use crs::Crs;
pub use div_icon::{DivIcon, DivIconOptions};
pub use div_overlay::DivOverlay;
pub use event::Event;
pub use evented::{
    DragEvents, Evented, EventedHandle, LayerEvents, MouseEvents, MoveEvents, PopupEvents,
    TooltipEvents,
};
pub use feature_group::FeatureGroup;
pub use geo_json::GeoJson;
pub use grid_layer::{GridLayer, GridLayerOptions};
pub use handler::Handler;
pub use icon::{Icon, IconOptions};
pub use lat_lng::LatLng;
pub use lat_lng_bounds::LatLngBounds;
pub use layer::{Layer, LayerOptions};
pub use layer_group::LayerGroup;
pub use map::{
    DragEndEvent, ErrorEvent, LocateOptions, LocationEvent, Map, MapOptions, MouseEvent,
    PopupEvent, TooltipEvent,
};
pub use marker::{Marker, MarkerOptions};
pub use point::Point;
pub use popup::{Popup, PopupOptions};
pub use raster::{
    ImageOverlay, ImageOverlayOptions, TileLayer, TileLayerOptions, TileLayerWms,
    TileLayerWmsOptions, VideoOverlay, VideoOverlayOptions, WmsRequestBuilder,
};
pub use shapes::{
    Circle, CircleMarker, CircleOptions, Path, PathOptions, Polygon, Polyline, PolylineOptions,
    Rectangle,
};
pub use tooltip::{Tooltip, TooltipOptions};
pub use util::Util;

#[macro_export]
macro_rules! object_property_set {
    ($a:ident, $b:ty) => {
        $crate::paste! {
            pub fn [<set_ $a>](&mut self, val: $b) {
                let _ = js_sys::Reflect::set(
                    self.as_ref(),
                    &wasm_bindgen::JsValue::from(stringify!($a)),
                    &wasm_bindgen::JsValue::from(val),
                );
            }
        }
    };
    ($a:ident, $b:ident, $c:ty) => {
        $crate::paste! {
            pub fn [<set_ $a>](&mut self, val: $c) {
                let _ = js_sys::Reflect::set(
                    self.as_ref(),
                    &wasm_bindgen::JsValue::from(stringify!($b)),
                    &wasm_bindgen::JsValue::from(val),
                );
            }
        }
    };
}

#[macro_export]
macro_rules! create_object_with_properties {
    (($t:ident, $t_js:ident), $(($rust:ident, $js:ident, $b:ty)),+) => {
        $crate::paste! {
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen (extends = Object , js_name = $t_js)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub type $t;

                $(
                #[wasm_bindgen(method, getter, js_name = $js)]
                pub fn $rust(this: &$t) -> $b;
                )*

                $(
                #[wasm_bindgen(method, setter, js_name = $js)]
                pub fn [<set_ $rust>](this: &$t, val: $b);
                )*
            }
        }
        impl $t {
            #[allow(clippy::new_without_default)]
            #[must_use] pub fn new() -> Self {
                #[allow(unused_mut)]
                let mut r = JsCast::unchecked_into(Object::new());
                r
            }
        }
    };
    (($t:ident, $t_js:ident, $t_extends:ident), $(($rust:ident, $js:ident, $b:ty)),+) => {
        $crate::paste! {
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(extends = $t_extends, js_name = $t_js)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub type $t;

                $(
                #[wasm_bindgen(method, getter, js_name = $js)]
                pub fn $rust(this: &$t) -> $b;
                )*

                $(
                #[wasm_bindgen(method, setter, js_name = $js)]
                pub fn [<set_ $rust>](this: &$t, val: $b);
                )*
            }
        }
        impl $t {
            #[allow(clippy::new_without_default)]
            #[must_use] pub fn new() -> Self {
                #[allow(unused_mut)]
                let mut r = JsCast::unchecked_into(Object::new());
                r
            }
        }
    };
}

#[macro_export]
macro_rules! add_object_properties {
    ($t:ident, $(($rust:ident, $js:ident, $b:ty)),+) => {
        $crate::paste! {
            #[wasm_bindgen]
            extern "C" {
                $(
                #[wasm_bindgen(method, getter, js_name = $js)]
                pub fn $rust(this: &$t) -> $b;
                )*

                $(
                #[wasm_bindgen(method, setter, js_name = $js)]
                pub fn [<set_ $rust>](this: &$t, val: $b);
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! object_property_set_with {
    ($a:ident, $b:ident, $c:expr) => {
        $crate::paste! {
            pub fn [<set_ $a>](&mut self) {
                let _ = js_sys::Reflect::set(
                    self.as_ref(),
                    &wasm_bindgen::JsValue::from(stringify!($b)),
                    &wasm_bindgen::JsValue::from($c),
                );
            }
        }
    };
}

#[macro_export]
macro_rules! object_constructor {
    () => {
        #[allow(clippy::new_without_default)]
        #[must_use]
        pub fn new() -> Self {
            #[allow(unused_mut)]
            let mut r = JsCast::unchecked_into(Object::new());
            r
        }
    };
}

pub fn to_lat_lng_array<T: Into<LatLng> + Clone>(lat_lngs: &[T]) -> Array {
    let array = Array::new();
    for lat_lng in lat_lngs.iter().cloned() {
        array.push(&lat_lng.into());
    }
    array
}
