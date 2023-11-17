use crate::{create_object_with_properties, Point};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_namespace = L, js_name = Icon)]
    pub type Icon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &IconOptions) -> Icon;
}

create_object_with_properties!(
    (IconOptions, IconOptions),
    (icon_url, iconUrl, String),
    (icon_size, iconSize, Point),
    (icon_anchor, iconAnchor, Point),
    (popup_anchor, popupAnchor, Point),
    (shadow_anchor, shadowAnchor, Point),
    (tooltip_anchor, tooltipAnchor, Point),
    (shadow_url, shadowUrl, String),
    (shadow_retina_url, shadowRetinaUrl, String),
    (shadow_size, shadowSize, Point),
    (class_name, className, String),
    (cross_origin, crossOrigin, String)
);

impl Default for IconOptions {
    fn default() -> Self {
        Self::new()
    }
}
