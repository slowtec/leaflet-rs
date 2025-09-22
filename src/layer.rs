use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::evented::{LayerEvents, LeafletEventHandler, PopupEvents, TooltipEvents};
use crate::{Evented, LatLng, LayerGroup, Map, Popup, Tooltip, create_object_with_properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Evented)]
    #[derive(Debug, Clone, PartialEq)]
    pub type Layer;

    /// [`addTo`](https://leafletjs.com/reference.html#layer-addto)
    #[wasm_bindgen(method, js_name = addTo)]
    pub fn add_to(this: &Layer, map: &Map) -> Layer;

    /// [`addTo`](https://leafletjs.com/reference.html#layer-addto)
    #[wasm_bindgen(method, js_name = addTo)]
    pub fn add_to_layer_group(this: &Layer, layerGroup: &LayerGroup) -> Layer;

    #[wasm_bindgen(method, js_name = remove)]
    pub fn remove(this: &Layer) -> Layer;

    #[wasm_bindgen(method, js_name = removeFrom)]
    pub fn remove_from(this: &Layer, map: &Map) -> Layer;

    #[wasm_bindgen(method, js_name = removeFrom)]
    pub fn remove_from_layer_group(this: &Layer, map: &LayerGroup) -> Layer;

    #[wasm_bindgen(method, js_name = getPane)]
    pub fn get_pane(this: &Layer) -> HtmlElement;

    #[wasm_bindgen(method, js_name = getAttribution)]
    pub fn get_attribution(this: &Layer) -> String;

    // Layer Popup Methods

    /// [`bindPopup`](https://leafletjs.com/reference.html#layer-bindpopup)
    #[wasm_bindgen(method, js_name = bindPopup)]
    pub fn bind_popup(this: &Layer, content: &Popup) -> Layer;

    /// [`bindPopup`](https://leafletjs.com/reference.html#layer-bindpopup)
    #[wasm_bindgen(method, js_name = bindPopup)]
    pub fn bind_popup_with_options(this: &Layer, content: &JsValue, options: &JsValue) -> Layer;

    /// [`unbindPopup`](https://leafletjs.com/reference.html#layer-unbindpopup)
    #[wasm_bindgen(method, js_name = unbindPopup)]
    pub fn unbind_popup(this: &Layer) -> Layer;

    /// [`openPopup`](https://leafletjs.com/reference.html#layer-openpopup)
    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn open_popup(this: &Layer) -> Layer;

    /// [`openPopup`](https://leafletjs.com/reference.html#layer-openpopup)
    #[wasm_bindgen(method, js_name = openPopup)]
    pub fn open_popup_with_lat_lng(this: &Layer, lat_lng: &LatLng) -> Layer;

    /// [`closePopup`](https://leafletjs.com/reference.html#layer-closepopup)
    #[wasm_bindgen(method, js_name = closePopup)]
    pub fn close_popup(this: &Layer) -> Layer;

    /// [`togglePopup`](https://leafletjs.com/reference.html#layer-togglepopup)
    #[wasm_bindgen(method, js_name = togglePopup)]
    pub fn toggle_popup(this: &Layer) -> Layer;

    /// [`isPopupOpen`](https://leafletjs.com/reference.html#layer-ispopupopen)
    #[wasm_bindgen(method, js_name = isPopupOpen)]
    pub fn is_popup_open(this: &Layer) -> bool;

    /// [`setPopupContent`](https://leafletjs.com/reference.html#layer-setpopupcontent)
    #[wasm_bindgen(method, js_name = setPopupContent)]
    pub fn set_popup_content(this: &Layer, content: &JsValue) -> Layer;

    /// [`getPopup`](https://leafletjs.com/reference.html#layer-getpopup)
    #[wasm_bindgen(method, js_name = getPopup)]
    pub fn get_popup(this: &Layer) -> Popup;

    // Layer Tooltip Methods

    /// [`bindTooltip`](https://leafletjs.com/reference.html#layer-bindtooltip)
    #[wasm_bindgen(method, js_name = bindTooltip)]
    pub fn bind_tooltip(this: &Layer, tooltip: &Tooltip) -> Layer;

    #[wasm_bindgen(method, js_name = bindTooltipWithContent)]
    pub fn bind_tooltip_with_content(this: &Layer, content: &JsValue, options: &JsValue) -> Layer;

    /// [`unbindTooltip`](https://leafletjs.com/reference.html#layer-unbindtooltip)
    #[wasm_bindgen(method, js_name = unbindTooltip)]
    pub fn unbind_tooltip(this: &Layer) -> Layer;

    /// [`openTooltip`](https://leafletjs.com/reference.html#layer-opentooltip)
    #[wasm_bindgen(method, js_name = openTooltip)]
    pub fn open_tooltip(this: &Layer, lat_lng: &LatLng) -> Layer;

    /// [`closeTooltip`](https://leafletjs.com/reference.html#layer-closetooltip)
    #[wasm_bindgen(method, js_name = closeTooltip)]
    pub fn close_tooltip(this: &Layer) -> Layer;

    /// [`toggleTooltip`](https://leafletjs.com/reference.html#layer-toggletooltip)
    #[wasm_bindgen(method, js_name = toggleTooltip)]
    pub fn toggle_tooltip(this: &Layer) -> Layer;

    /// [`isTooltipOpen`](https://leafletjs.com/reference.html#layer-istooltipopen)
    #[wasm_bindgen(method, js_name = isTooltipOpen)]
    pub fn is_tooltip_open(this: &Layer) -> bool;

    /// [`setTooltipContent`](https://leafletjs.com/reference.html#layer-settooltipcontent)
    #[wasm_bindgen(method, js_name = setTooltipContent)]
    pub fn set_tooltip_content(this: &Layer, content: &JsValue) -> Layer;

    /// [`getTooltip`](https://leafletjs.com/reference.html#layer-gettooltip)
    #[wasm_bindgen(method, js_name = getTooltip)]
    pub fn get_tooltip(this: &Layer) -> Tooltip;
}

create_object_with_properties!(
    (LayerOptions, LayerOptions),
    (pane, pane, String),
    (attribution, attribution, String)
);

impl LeafletEventHandler for Layer {
    fn on(&self, event: &str, callback: &JsValue) {
        self.unchecked_ref::<Evented>().on(event, callback);
    }
}

impl LayerEvents for Layer {}
impl PopupEvents for Layer {}
impl TooltipEvents for Layer {}
