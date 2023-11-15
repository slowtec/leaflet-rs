use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::evented::{LayerEvents, LeafletEventHandler, PopupEvents, TooltipEvents};
use crate::{
    object_constructor, object_property_set, Evented, LatLng, LayerGroup, Map, Popup, Tooltip,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = LayerOptions)]
    #[derive(Debug, Clone, PartialEq)]
    pub type LayerOptions;

    #[wasm_bindgen(extends = Evented)]
    #[derive(Debug, Clone, PartialEq)]
    pub type Layer;

    /// [`addTo`](https://leafletjs.com/reference.html#layer-addto)
    #[wasm_bindgen(method, js_name = addTo)]
    pub fn addTo(this: &Layer, map: &Map) -> Layer;

    /// [`addTo`](https://leafletjs.com/reference.html#layer-addto)
    #[wasm_bindgen(method, js_name = addTo)]
    pub fn addToLayerGroup(this: &Layer, layerGroup: &LayerGroup) -> Layer;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &Layer) -> Layer;

    #[wasm_bindgen(method, js_name = removeFrom)]
    pub fn removeFrom(this: &Layer, map: &Map) -> Layer;

    #[wasm_bindgen(method, js_name = removeFrom)]
    pub fn removeFromLayerGroup(this: &Layer, map: &LayerGroup) -> Layer;

    #[wasm_bindgen(method, js_name = getPane)]
    pub fn getPane(this: &Layer) -> HtmlElement;

    #[wasm_bindgen(method, js_name = getAttribution)]
    pub fn getAttribution(this: &Layer) -> String;

    // Layer Popup Methods

    /// [`bindPopup`](https://leafletjs.com/reference.html#layer-bindpopup)
    #[wasm_bindgen(method)]
    pub fn bindPopup(this: &Layer, content: &Popup) -> Layer;

    /// [`bindPopup`](https://leafletjs.com/reference.html#layer-bindpopup)
    #[wasm_bindgen(method)]
    pub fn bindPopupWithOptions(this: &Layer, content: &JsValue, options: &JsValue) -> Layer;

    /// [`unbindPopup`](https://leafletjs.com/reference.html#layer-unbindpopup)
    #[wasm_bindgen(method)]
    pub fn unbindPopup(this: &Layer) -> Layer;

    /// [`openPopup`](https://leafletjs.com/reference.html#layer-openpopup)
    #[wasm_bindgen(method)]
    pub fn openPopup(this: &Layer) -> Layer;

    /// [`openPopup`](https://leafletjs.com/reference.html#layer-openpopup)
    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn openPopupWithLatLng(this: &Layer, lat_lng: &LatLng) -> Layer;

    /// [`closePopup`](https://leafletjs.com/reference.html#layer-closepopup)
    #[wasm_bindgen(method)]
    pub fn closePopup(this: &Layer) -> Layer;

    /// [`togglePopup`](https://leafletjs.com/reference.html#layer-togglepopup)
    #[wasm_bindgen(method)]
    pub fn togglePopup(this: &Layer) -> Layer;

    /// [`isPopupOpen`](https://leafletjs.com/reference.html#layer-ispopupopen)
    #[wasm_bindgen(method)]
    pub fn isPopupOpen(this: &Layer) -> bool;

    /// [`setPopupContent`](https://leafletjs.com/reference.html#layer-setpopupcontent)
    #[wasm_bindgen(method)]
    pub fn setPopupContent(this: &Layer, content: &JsValue) -> Layer;

    /// [`getPopup`](https://leafletjs.com/reference.html#layer-getpopup)
    #[wasm_bindgen(method)]
    pub fn getPopup(this: &Layer) -> Popup;

    // Layer Tooltip Methods

    /// [`bindTooltip`](https://leafletjs.com/reference.html#layer-bindtooltip)
    #[wasm_bindgen(method)]
    pub fn bindTooltip(this: &Layer, tooltip: &Tooltip) -> Layer;

    #[wasm_bindgen(method)]
    pub fn bindTooltipWithContent(this: &Layer, content: &JsValue, options: &JsValue) -> Layer;

    /// [`unbindTooltip`](https://leafletjs.com/reference.html#layer-unbindtooltip)
    #[wasm_bindgen(method)]
    pub fn unbindTooltip(this: &Layer) -> Layer;

    /// [`openTooltip`](https://leafletjs.com/reference.html#layer-opentooltip)
    #[wasm_bindgen(method)]
    pub fn openTooltip(this: &Layer, lat_lng: &LatLng) -> Layer;

    /// [`closeTooltip`](https://leafletjs.com/reference.html#layer-closetooltip)
    #[wasm_bindgen(method)]
    pub fn closeTooltip(this: &Layer) -> Layer;

    /// [`toggleTooltip`](https://leafletjs.com/reference.html#layer-toggletooltip)
    #[wasm_bindgen(method)]
    pub fn toggleTooltip(this: &Layer) -> Layer;

    /// [`isTooltipOpen`](https://leafletjs.com/reference.html#layer-istooltipopen)
    #[wasm_bindgen(method)]
    pub fn isTooltipOpen(this: &Layer) -> bool;

    /// [`setTooltipContent`](https://leafletjs.com/reference.html#layer-settooltipcontent)
    #[wasm_bindgen(method)]
    pub fn setTooltipContent(this: &Layer, content: &JsValue) -> Layer;

    /// [`getTooltip`](https://leafletjs.com/reference.html#layer-gettooltip)
    #[wasm_bindgen(method)]
    pub fn getTooltip(this: &Layer) -> Tooltip;
}

impl LayerOptions {
    object_constructor!();
    object_property_set!(pane, &str);
    object_property_set!(attribution, &str);
}

impl LeafletEventHandler for Layer {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl LayerEvents for Layer {}
impl PopupEvents for Layer {}
impl TooltipEvents for Layer {}
