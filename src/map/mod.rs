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
    pub fn new_with_element(el: &HtmlElement, options: &MapOptions) -> Map;

    // Methods for Layers and Controls
    #[wasm_bindgen(method, js_name = addControl)]
    pub fn add_control(this: &Map, control: &Control) -> Map;

    #[wasm_bindgen(method, js_name = removeControl)]
    pub fn remove_control(this: &Map, control: &Control) -> Map;

    #[wasm_bindgen(method, js_name = addLayer)]
    pub fn add_layer(this: &Map, layer: &Layer) -> Map;

    #[wasm_bindgen(method, js_name = removeLayer)]
    pub fn remove_layer(this: &Map, layer: &Layer) -> Map;

    #[wasm_bindgen(method, js_name = hasLayer)]
    pub fn has_layer(this: &Map, layer: &Layer) -> bool;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn each_layer(this: &Map, for_each: &dyn Fn(Layer)) -> Map;

    #[wasm_bindgen(method, js_name = eachLayer)]
    pub fn each_layer_with_context(this: &Map, for_each: &dyn Fn(Layer), context: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn open_popup(this: &Map, popup: &Popup) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn open_popup_with_content(this: &Map, content: &JsValue, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn open_popup_with_content_and_options(
        this: &Map,
        content: &JsValue,
        lat_lng: &LatLng,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = closePopup)]
    pub fn close_popup(this: &Map, popup: &Popup) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn open_tooltip(this: &Map, tooltip: &Tooltip) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn open_tooltip_with_content(this: &Map, content: &JsValue, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn open_tooltip_with_content_and_options(
        this: &Map,
        content: &JsValue,
        lat_lng: &LatLng,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = closeTooltip)]
    pub fn close_tooltip(this: &Map, tooltip: &Tooltip) -> Map;

    // Methods for modifying map state

    #[wasm_bindgen(method, js_name = setView)]
    pub fn set_view(this: &Map, center: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setView)]
    pub fn set_view_with_options(this: &Map, center: &LatLng, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = getBounds)]
    pub fn get_bounds(this: &Map) -> LatLngBounds;

    #[wasm_bindgen(method, js_name = getCenter)]
    pub fn get_center(this: &Map) -> LatLng;

    #[wasm_bindgen(method, js_name = getZoom)]
    pub fn get_zoom(this: &Map) -> f64;

    #[wasm_bindgen(method, js_name = getZoomScale)]
    pub fn get_zoom_scale(this: &Map, toZoom: f64, fromZoom: f64) -> f64;

    #[wasm_bindgen(method, js_name = setZoom)]
    pub fn set_zoom(this: &Map, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoom)]
    pub fn set_zoom_with_options(this: &Map, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = zoomIn)]
    pub fn zoom_in(this: &Map, delta: f64) -> Map;

    #[wasm_bindgen(method, js_name = zoomIn)]
    pub fn zoom_in_with_options(this: &Map, delta: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = zoomOut)]
    pub fn zoom_out(this: &Map, delta: f64);

    #[wasm_bindgen(method, js_name = zoomOut)]
    pub fn zoom_out_with_options(this: &Map, delta: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn set_zoom_around_lat_lng(this: &Map, latlng: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn set_zoom_around_lat_lng_with_options(
        this: &Map,
        latlng: &LatLng,
        zoom: f64,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn set_zoom_around_point(this: &Map, offset: &Point, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setZoomAround)]
    pub fn set_zoom_around_point_with_options(
        this: &Map,
        offset: &Point,
        zoom: f64,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = "fitBounds")]
    pub fn fit_bounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = fitBounds)]
    pub fn fit_bounds_with_options(this: &Map, bounds: &LatLngBounds, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = fitWorld)]
    pub fn fit_world(this: &Map) -> Map;

    #[wasm_bindgen(method, js_name = fitWorld)]
    pub fn fit_world_with_options(this: &Map, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = panTo)]
    pub fn pan_to(this: &Map, lat_lng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = panTo)]
    pub fn pan_to_with_options(this: &Map, lat_lng: &LatLng, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = panBy)]
    pub fn pan_by(this: &Map, point: &Point) -> Map;

    #[wasm_bindgen(method, js_name = panBy)]
    pub fn pan_by_with_options(this: &Map, point: &Point, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn fly_to(this: &Map, lat_lng: &LatLng, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = flyTo)]
    pub fn fly_to_with_options(this: &Map, latlng: &LatLng, zoom: f64, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = flyToBounds)]
    pub fn fly_to_bounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = flyToBounds)]
    pub fn fly_to_bounds_with_options(this: &Map, bounds: &LatLngBounds, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = setMaxBounds)]
    pub fn set_max_bounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = setMinZoom)]
    pub fn set_min_zoom(this: &Map, zoom: f64) -> Map;

    #[wasm_bindgen(method, js_name = setMaxZoom)]
    pub fn set_max_zoom(this: &Map, zoom: f64) -> Map;

    /// [`getMaxZoom`](https://leafletjs.com/reference-1.7.1.html#map-getmaxzoom)
    #[wasm_bindgen(method, js_name = getMaxZoom)]
    pub fn get_max_zoom(this: &Map) -> f64;

    #[wasm_bindgen(method, js_name = panInsideBounds)]
    pub fn pan_inside_bounds(this: &Map, bounds: &LatLngBounds) -> Map;

    #[wasm_bindgen(method, js_name = panInsideBounds)]
    pub fn pan_inside_bounds_with_options(
        this: &Map,
        bounds: &LatLngBounds,
        options: &JsValue,
    ) -> Map;

    #[wasm_bindgen(method, js_name = panInside)]
    pub fn pan_inside(this: &Map, latlng: &LatLng) -> Map;

    #[wasm_bindgen(method, js_name = panInside)]
    pub fn pan_inside_with_options(this: &Map, latlng: &LatLng, options: &JsValue) -> Map;

    #[wasm_bindgen(method, js_name = invalidateSize)]
    pub fn invalidate_size(this: &Map, animate: bool) -> Map;

    #[wasm_bindgen(method, js_name = invalidateSize)]
    pub fn invalidate_size_with_options(this: &Map, options: &JsValue) -> Map;

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

    #[wasm_bindgen(method, js_name = latLngToContainerPoint)]
    pub fn lat_lng_to_container_point(this: &Map, latlng: &LatLng) -> Point;

    #[wasm_bindgen(method, js_name = containerPointToLatLng)]
    pub fn container_point_to_lat_lng(this: &Map, point: &Point) -> LatLng;

    #[wasm_bindgen(method, js_name = layerPointToLatLng)]
    pub fn layer_point_to_lat_lng(this: &Map, point: &Point) -> LatLng;

    #[wasm_bindgen(method, js_name = latLngToLayerPoint)]
    pub fn lat_lng_to_layer_point(this: &Map, latlng: &LatLng) -> Point;
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
