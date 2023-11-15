use crate::{object_constructor, object_property_set, Point};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, js_name = IconOptions)]
    #[derive(Debug, Clone)]
    pub type IconOptions;

    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_namespace = L, js_name = Icon)]
    pub type Icon;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(options: &IconOptions) -> Icon;
}

impl IconOptions {
    object_constructor!();
    object_property_set!(icon_url, iconUrl, &str);
    object_property_set!(icon_size, iconSize, Point);
    object_property_set!(icon_anchor, iconAnchor, Point);
    object_property_set!(popup_anchor, popupAnchor, Point);
    object_property_set!(shadow_anchor, shadowAnchor, Point);
    object_property_set!(tooltip_anchor, tooltipAnchor, Point);
    object_property_set!(shadow_url, shadowUrl, &str);
    object_property_set!(shadow_retina_url, shadowRetinaUrl, &str);
    object_property_set!(shadow_size, shadowSize, Point);
    object_property_set!(class_name, className, &str);
    object_property_set!(cross_origin, crossOrigin, &str);
}

impl Default for IconOptions {
    fn default() -> Self {
        Self::new()
    }
}
