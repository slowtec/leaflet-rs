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

    // Layer Popup Methods

    /// [`bindPopup`](https://leafletjs.com/reference-1.7.1.html#layer-bindpopup)
    #[wasm_bindgen(method)]
    pub fn bindPopup(this: &Layer, content: &JsValue, options: &JsValue);

    /// [`unbindPopup`](https://leafletjs.com/reference-1.7.1.html#layer-unbindpopup)
    #[wasm_bindgen(method)]
    pub fn unbindPopup(this: &Layer);

    /// [`openPopup`](https://leafletjs.com/reference-1.7.1.html#layer-openpopup)
    #[wasm_bindgen(method)]
    pub fn openPopup(this: &Layer, latlng: &LatLng);

    /// [`closePopup`](https://leafletjs.com/reference-1.7.1.html#layer-closepopup)
    #[wasm_bindgen(method)]
    pub fn closePopup(this: &Layer);

    /// [`togglePopup`](https://leafletjs.com/reference-1.7.1.html#layer-togglepopup)
    #[wasm_bindgen(method)]
    pub fn togglePopup(this: &Layer);

    /// [`isPopupOpen`](https://leafletjs.com/reference-1.7.1.html#layer-ispopupopen)
    #[wasm_bindgen(method)]
    pub fn isPopupOpen(this: &Layer) -> bool;

    /// [`setPopupContent`](https://leafletjs.com/reference-1.7.1.html#layer-setpopupcontent)
    #[wasm_bindgen(method)]
    pub fn setPopupContent(this: &Layer, content: &JsValue);

    /// [`getPopup`](https://leafletjs.com/reference-1.7.1.html#layer-getpopup)
    #[wasm_bindgen(method)]
    pub fn getPopup(this: &Layer) -> Popup;

    // Layer Tooltip Methods

    /// [`bindTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-bindtooltip)
    #[wasm_bindgen(method)]
    pub fn bindTooltip(this: &Layer, content: &JsValue, options: &JsValue);

    /// [`unbindTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-unbindtooltip)
    #[wasm_bindgen(method)]
    pub fn unbindTooltip(this: &Layer);

    /// [`openTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-opentooltip)
    #[wasm_bindgen(method)]
    pub fn openTooltip(this: &Layer, latlng: &LatLng);

    /// [`closeTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-closetooltip)
    #[wasm_bindgen(method)]
    pub fn closeTooltip(this: &Layer);

    /// [`toggleTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-toggletooltip)
    #[wasm_bindgen(method)]
    pub fn toggleTooltip(this: &Layer);

    /// [`isTooltipOpen`](https://leafletjs.com/reference-1.7.1.html#layer-istooltipopen)
    #[wasm_bindgen(method)]
    pub fn isTooltipOpen(this: &Layer) -> bool;

    /// [`setTooltipContent`](https://leafletjs.com/reference-1.7.1.html#layer-settooltipcontent)
    #[wasm_bindgen(method)]
    pub fn setTooltipContent(this: &Layer, content: &JsValue);

    /// [`getTooltip`](https://leafletjs.com/reference-1.7.1.html#layer-gettooltip)
    #[wasm_bindgen(method)]
    pub fn getTooltip(this: &Layer) -> Tooltip;

    // LayerGroup

    /// [`LayerGroup`](https://leafletjs.com/reference-1.7.1.html#layergroup)
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type LayerGroup;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new() -> LayerGroup;

    /// [`toGeoJSON`](https://leafletjs.com/reference-1.7.1.html#layergroup-togeojson)
    #[wasm_bindgen(method)]
    pub fn toGeoJSON(this: &LayerGroup) -> JsValue;

    /// [`addLayer`](https://leafletjs.com/reference-1.7.1.html#layergroup-addlayer)
    #[wasm_bindgen(method)]
    pub fn addLayer(this: &LayerGroup, layer: &Layer);

    /// [`removeLayer`](https://leafletjs.com/reference-1.7.1.html#layergroup-removelayer)
    #[wasm_bindgen(method)]
    pub fn removeLayer(this: &LayerGroup, layer: &Layer);

    /// [`hasLayer`](https://leafletjs.com/reference-1.7.1.html#layergroup-haslayer)
    #[wasm_bindgen(method)]
    pub fn hasLayer(this: &LayerGroup, layer: &Layer) -> bool;

    /// [`clearLayers`](https://leafletjs.com/reference-1.7.1.html#layergroup-clearlayers)
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
    pub fn setZoomAroundLatLngWithOptions(
        this: &Map,
        latlng: &LatLng,
        zoom: f64,
        options: &JsValue,
    );

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

    // Popup

    /// [`Popup`](https://leafletjs.com/reference-1.7.1.html#popup)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Popup;

    /// [`L.popup`](https://leafletjs.com/reference-1.7.1.html#popup-l-popup)
    #[wasm_bindgen(js_namespace = L)]
    pub fn popup(options: &JsValue, layer: Option<&Layer>) -> Popup;

    /// [`getLatLng`](https://leafletjs.com/reference-1.7.1.html#popup-getlatlng)
    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &Popup) -> LatLng;

    /// [`setLatLng`](https://leafletjs.com/reference-1.7.1.html#popup-setlatlng)
    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &Popup, latlng: &LatLng);

    /// [`getContent`](https://leafletjs.com/reference-1.7.1.html#popup-getcontent)
    #[wasm_bindgen(method)]
    pub fn getContent(this: &Popup) -> JsValue;

    /// [`setContent`](https://leafletjs.com/reference-1.7.1.html#popup-setcontent)
    #[wasm_bindgen(method)]
    pub fn setContent(this: &Popup, content: &JsValue);

    /// [`update`](https://leafletjs.com/reference-1.7.1.html#popup-update)
    #[wasm_bindgen(method)]
    pub fn update(this: &Popup);

    /// [`isOpen`](https://leafletjs.com/reference-1.7.1.html#popup-isopen)
    #[wasm_bindgen(method)]
    pub fn isOpen(this: &Popup) -> bool;

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#popup-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &Popup);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#popup-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &Popup);

    /// [`openOn`](https://leafletjs.com/reference-1.7.1.html#popup-openon)
    #[wasm_bindgen(method)]
    pub fn openOn(this: &Popup, map: &Map);

    // Tooltip

    /// [`Tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip)
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Layer)]
    pub type Tooltip;

    /// [`L.tooltip`](https://leafletjs.com/reference-1.7.1.html#tooltip-l-tooltip)
    #[wasm_bindgen(js_namespace = L)]
    pub fn tooltip(options: &JsValue, layer: Option<&Layer>) -> Popup;

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

    /// [`Polyline`](https://leafletjs.com/reference-1.7.1.html#polyline)
    #[derive(Debug)]
    #[wasm_bindgen(extends = Path)]
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

    // CircleMarker

    /// [`CirleMarker`](https://leafletjs.com/reference-1.7.1.html#circlemarker)
    #[derive(Debug)]
    #[wasm_bindgen(extends = Path)]
    pub type CircleMarker;

    /// [`Constructor`](https://leafletjs.com/reference-1.7.1.html#circlemarker-l-circlemarker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> CircleMarker;

    /// [`Constructor`](https://leafletjs.com/reference-1.7.1.html#circlemarker-l-circlemarker)
    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> CircleMarker;

    /// [`toGeoJSON`](https://leafletjs.com/reference-1.7.1.html#circlemarker-togeojson)
    #[wasm_bindgen(method)]
    pub fn toGeoJSON(this: &CircleMarker) -> JsValue;

    /// [`setLatLng`](https://leafletjs.com/reference-1.7.1.html#circlemarker-setlanglng)
    #[wasm_bindgen(method)]
    pub fn setLatLng(this: &CircleMarker, latlng: &LatLng);

    /// [`getLatLng`](https://leafletjs.com/reference-1.7.1.html#circlemarker-getlatlng)
    #[wasm_bindgen(method)]
    pub fn getLatLng(this: &CircleMarker) -> LatLng;

    /// [`setRadius`](https://leafletjs.com/reference-1.7.1.html#circlemarker-setradius)
    #[wasm_bindgen(method)]
    pub fn setRadius(this: &CircleMarker, radius: f64);

    /// [`getRadius`](https://leafletjs.com/reference-1.7.1.html#circlemarker-getradius)
    #[wasm_bindgen(method)]
    pub fn getRadius(this: &CircleMarker) -> f64;

    // Circle

    #[derive(Debug)]
    #[wasm_bindgen(extends = CircleMarker)]
    pub type Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(latlng: &LatLng) -> Circle;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new_with_options(latlng: &LatLng, options: &JsValue) -> Circle;

    /// [`setRadius`](https://leafletjs.com/reference-1.7.1.html#circle-setradius)
    #[wasm_bindgen(method)]
    pub fn setRadius(this: &Circle, radius: f64);

    /// [`getRadius`](https://leafletjs.com/reference-1.7.1.html#circle-getradius)
    #[wasm_bindgen(method)]
    pub fn getRadius(this: &Circle) -> f64;

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


    // Path

    /// [`Path`](https://leafletjs.com/reference-1.7.1.html#path)
    #[derive(Debug)]
    #[wasm_bindgen(extends = Layer)]
    pub type Path;

    /// [`redraw`](https://leafletjs.com/reference-1.7.1.html#path-redraw)
    #[wasm_bindgen(method)]
    pub fn redraw(this: &Path);

    /// [`setStyle`](https://leafletjs.com/reference-1.7.1.html#path-setstyle)
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &Path, path_options: &JsValue);

    /// [`bringToFront`](https://leafletjs.com/reference-1.7.1.html#path-bringtofront)
    #[wasm_bindgen(method)]
    pub fn bringToFront(this: &Path);

    /// [`bringToBack`](https://leafletjs.com/reference-1.7.1.html#path-bringtoback)
    #[wasm_bindgen(method)]
    pub fn bringToBack(this: &Path);
}
