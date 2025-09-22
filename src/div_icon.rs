use crate::{Icon, Point, create_object_with_properties};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Icon, js_namespace = L, js_name = Icon)]
    pub type DivIcon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &DivIconOptions) -> DivIcon;
}

create_object_with_properties!(
    (DivIconOptions, DivIconOptions),
    (html, html, String),
    (bg_pos, bgPos, Point),
    (icon_size, iconSize, Point),
    (icon_anchor, iconAnchor, Point),
    (popup_anchor, popupAnchor, Point),
    (tooltip_anchor, tooltipAnchor, Point),
    (class_name, className, String),
    (cross_origin, crossOrigin, String)
);

impl Default for DivIconOptions {
    fn default() -> Self {
        Self::new()
    }
}
