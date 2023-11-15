use crate::{object_constructor, object_property_set, Icon, Point};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = DivIconOptions)]
    #[derive(Debug, Clone)]
    pub type DivIconOptions;

    #[derive(Debug, Clone)]
    #[wasm_bindgen(extends = Icon, js_namespace = L, js_name = Icon)]
    pub type DivIcon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &DivIconOptions) -> DivIcon;
}

impl DivIconOptions {
    object_constructor!();
    object_property_set!(html, &str);
    object_property_set!(bg_pos, bgPos, Point);
    object_property_set!(icon_size, iconSize, Point);
    object_property_set!(icon_anchor, iconAnchor, Point);
    object_property_set!(popup_anchor, popupAnchor, Point);
    object_property_set!(tooltip_anchor, tooltipAnchor, Point);
    object_property_set!(class_name, className, &str);
    object_property_set!(cross_origin, crossOrigin, &str);
}

impl Default for DivIconOptions {
    fn default() -> Self {
        Self::new()
    }
}
