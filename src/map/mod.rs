mod events;
mod geolocation;
mod location_event;
mod other;

use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{
    object_constructor, object_property_set, object_property_set_with, Control, Evented, LatLng,
    LatLngBounds, Layer, Point, Popup, Tooltip,
};
pub use events::*;
pub use geolocation::*;
pub use location_event::*;
pub use other::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object , js_name = MapOptions)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type MapOptions;

    #[wasm_bindgen(extends=Evented)]
    #[derive(Debug, Clone)]
    pub type Map;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(id: &str, options: &MapOptions) -> Map;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn newWithElement(el: &HtmlElement, options: &MapOptions) -> Map;

    // Methods for Layers and Controls
    #[wasm_bindgen(method, js_name = addControl)]
    pub fn addControl(this: &Map, control: &Control) -> Map;

    #[wasm_bindgen(method, js_name = removeControl)]
    pub fn removeControl(this: &Map, control: &Control) -> Map;

    #[wasm_bindgen(method, js_name = addLayer)]
    pub fn addLayer(this: &Map, layer: &Layer) -> Map;

    #[wasm_bindgen(method, js_name = removeLayer)]
    pub fn removeLayer(this: &Map, layer: &Layer) -> Map;

    #[wasm_bindgen(method, js_name = hasLayer)]
    pub fn hasLayer(this: &Map, layer: &Layer) -> bool;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn eachLayer(this: &Map, for_each: &dyn Fn(Layer)) -> Map;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn eachLayerWithContext(this: &Map, for_each: &dyn Fn(Layer), context: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn openPopup(this: &Map, popup: &Popup) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn openPopupWithContent(this: &Map, content: &JsValue, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn openPopupWithContentAndOptions(
        this: &Map,
        content: &JsValue,
        lat_lng: &LatLng,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = closePopup)]
    pub fn closePopup(this: &Map, popup: &Popup) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn openTooltip(this: &Map, tooltip: &Tooltip) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn openTooltipWithContent(this: &Map, content: &JsValue, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn openTooltipWithContentAndOptions(
        this: &Map,
        content: &JsValue,
        lat_lng: &LatLng,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = closeTooltip)]
    pub fn closeTooltip(this: &Map, tooltip: &Tooltip) -> Map;

    // Methods for modifying map state

    #[wasm_bindgen(method)]
    pub fn setView(this: &Map, center: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setView)]
    pub fn setViewWithOptions(this: &Map, center: &LatLng, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn getBounds(this: &Map) -> LatLngBounds;

    #[wasm_bindgen(method)]
    pub fn getCenter(this: &Map) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn getZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn getZoomScale(this: &Map, toZoom: f64, fromZoom: f64) -> f64;

    #[wasm_bindgen(method)]
    pub fn setZoom(this: &Map, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoom)]
    pub fn setZoomWithOptions(this: &Map, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn zoomIn(this: &Map, delta: f64) -> Map;

    #[wasm_bindgen(method, js_name = zoomIn)]
    pub fn zoomInWithOptions(this: &Map, delta: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn zoomOut(this: &Map, delta: f64);

    #[wasm_bindgen(method, js_name = zoomOut)]
    pub fn zoomOutWithOptions(this: &Map, delta: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundLatLng(this: &Map, latlng: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundLatLngWithOptions(
        this: &Map,
        latlng: &LatLng,
        zoom: f64,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundPoint(this: &Map, offset: &Point, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn setZoomAroundPointWithOptions(
        this: &Map,
        offset: &Point,
        zoom: f64,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method)]
    pub fn fitBounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = fitBounds)]
    pub fn fitBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn fitWorld(this: &Map) -> Map;

    #[wasm_bindgen(method, js_name = fitWorld)]
    pub fn fitWorldWithOptions(this: &Map, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn panTo(this: &Map, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = panTo)]
    pub fn panToWithOptions(this: &Map, lat_lng: &LatLng, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn panBy(this: &Map, point: &Point) -> Map;

    #[wasm_bindgen(method, js_name = panBy)]
    pub fn panByWithOptions(this: &Map, point: &Point, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn flyTo(this: &Map, lat_lng: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn flyToWithOptions(this: &Map, latlng: &LatLng, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn flyToBounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = flyToBounds)]
    pub fn flyToBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn setMaxBounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method)]
    pub fn setMinZoom(this: &Map, zoom: f64) -> Map;

    #[wasm_bindgen(method)]
    pub fn setMaxZoom(this: &Map, zoom: f64) -> Map;

    /// [`getMaxZoom`](https://leafletjs.com/reference-1.7.1.html#map-getmaxzoom)
    #[wasm_bindgen(method)]
    pub fn getMaxZoom(this: &Map) -> f64;

    #[wasm_bindgen(method)]
    pub fn panInsideBounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = panInsideBounds)]
    pub fn panInsideBoundsWithOptions(this: &Map, bounds: &LatLngBounds, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn panInside(this: &Map, latlng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = panInside)]
    pub fn panInsideWithOptions(this: &Map, latlng: &LatLng, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn invalidateSize(this: &Map, animate: bool) -> Map;

    #[wasm_bindgen(method, js_name = invalidateSize)]
    pub fn invalidateSizeWithOptions(this: &Map, options: &JsValue) -> Map;

    #[wasm_bindgen(method)]
    pub fn stop(this: &Map) -> Map;

    #[wasm_bindgen(method)]
    pub fn project(this: &Map, point: &LatLng) -> Point;

    #[wasm_bindgen(method)]
    pub fn unproject(this: &Map, point: &Point) -> LatLng;

    #[wasm_bindgen(method, js_name = project)]
    pub fn project_with_zoom(this: &Map, point: &LatLng, zoom: f64) -> Point;

    #[wasm_bindgen(method, js_name = unproject)]
    pub fn unproject_with_zoom(this: &Map, point: &Point, zoom: f64) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn distance(this: &Map, latlng1: &LatLng, latlng2: &LatLng) -> f64;

    #[wasm_bindgen(method)]
    pub fn latLngToContainerPoint(this: &Map, latlng: &LatLng) -> Point;

    #[wasm_bindgen(method)]
    pub fn containerPointToLatLng(this: &Map, point: &Point) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn layerPointToLatLng(this: &Map, point: &Point) -> LatLng;

    #[wasm_bindgen(method)]
    pub fn latLngToLayerPoint(this: &Map, latlng: &LatLng) -> Point;
}

impl MapOptions {
    object_constructor!();
    // Options
    object_property_set!(prefer_canvas, preferCanvas, bool);
    // Control options
    object_property_set!(attribution_control, attributionControl, bool);
    object_property_set!(zoom_control, zoomControl, bool);
    // Interaction Options
    object_property_set!(close_popup_on_click, closePopupOnClick, bool);
    object_property_set!(box_zoom, boxZoom, bool);
    object_property_set!(double_click_zoom, doubleClickZoom, JsValue);
    object_property_set!(dragging, bool);
    object_property_set!(zoom_snap, zoomSnap, f64);
    object_property_set!(zoom_delta, zoomDelta, f64);
    object_property_set!(track_resize, trackResize, bool);
    // Panning Inertia Options
    object_property_set!(inertia, bool);
    object_property_set!(inertia_deceleration, inertiaDeceleration, f64);
    object_property_set!(inertia_max_speed, inertiaMaxSpeed, f64);
    object_property_set!(ease_linearity, easeLinearity, f64);
    object_property_set!(world_copy_jump, worldCopyJump, bool);
    object_property_set!(max_bounds_viscosity, maxBoundsViscosity, f64);
    // Keyboard Navigation Options
    object_property_set!(keyboard, bool);
    object_property_set!(keyboard_pan_delta, keyboardPanDelta, f64);
    // Mouse wheel options
    object_property_set!(scroll_wheel_zoom, scrollWheelZoom, bool);
    object_property_set_with!(scroll_wheel_zoom_center, scrollWheelZoom, "center");
    object_property_set!(wheel_debounce_time, wheelDebounceTime, f64);
    object_property_set!(wheel_px_per_zoom_level, wheelPxPerZoomLevel, f64);
    // Touch interaction options
    object_property_set!(tap_hold, tapHold, bool);
    object_property_set!(tap_tolerance, tapTolerance, f64);
    object_property_set!(touch_zoom, touchZoom, bool);
    object_property_set_with!(touch_zoom_center, touchZoom, "center");
    object_property_set!(bounce_at_zoom_limits, bounceAtZoomLimits, bool);
    // Map State Options
    object_property_set!(crs, &JsValue);
    object_property_set!(center, &LatLng);
    object_property_set!(zoom, f64);
    object_property_set!(min_zoom, minZoom, f64);
    object_property_set!(max_zoom, maxZoom, f64);
    object_property_set!(layers, &Array);
    object_property_set!(max_bounds, maxBounds, &LatLngBounds);
    object_property_set!(renderer, &JsValue);
    // Animation Options
    object_property_set!(zoom_animation, zoomAnimation, bool);
    object_property_set!(zoom_animation_threshold, zoomAnimationThreshold, f64);
    object_property_set!(fade_animation, fadeAnimation, bool);
    object_property_set!(marker_zoom_animation, markerZoomAnimation, bool);
    object_property_set!(transform3d_limit, transform3DLimit, f64);
}

impl Default for MapOptions {
    fn default() -> Self {
        MapOptions::new()
    }
}
