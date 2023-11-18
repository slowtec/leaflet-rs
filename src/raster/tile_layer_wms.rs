use js_sys::Object;
use wasm_bindgen::prelude::*;

use crate::{create_object_with_properties, Crs, LatLng, Map, TileLayer, TileLayerOptions};

#[wasm_bindgen]
extern "C" {

    #[derive(Debug, Clone, PartialEq)]
    #[wasm_bindgen(extends = TileLayer, js_namespace = ["L", "tileLayer"], js_name = "TileLayerWMS")]
    pub type TileLayerWms;

    #[wasm_bindgen(js_namespace = ["L", "tileLayer"], js_name = "wms")]
    fn new_wms(url_template: &str) -> TileLayerWms;

    #[wasm_bindgen(js_namespace = ["L", "tileLayer"], js_name = "wms")]
    fn new_wms_options(url_template: &str, options: &TileLayerWmsOptions) -> TileLayerWms;

    #[wasm_bindgen(method, js_name = setParams)]
    pub fn set_params(
        this: &TileLayer,
        params: &TileLayerWmsOptions,
        no_redraw: Option<bool>,
    ) -> TileLayerWms;

}

impl TileLayerWms {
    #[must_use]
    pub fn new(url_template: &str) -> TileLayerWms {
        new_wms(url_template)
    }

    #[must_use]
    pub fn new_options(url_template: &str, options: &TileLayerWmsOptions) -> TileLayerWms {
        new_wms_options(url_template, options)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_params(&self) -> TileLayerWmsOptions {
        js_sys::Reflect::get(self.as_ref(), &wasm_bindgen::JsValue::from("wmsParams"))
            .expect("Property wmsParams not available.")
            .into()
    }

    /// Creates a feature info URL that can be used to query a feature from a WMS service.
    /// Based on [BetterWMS](https://github.com/sazal-ns/BetterWMS).
    #[must_use]
    pub fn get_feature_info_url(&self, map: &Map, latlng: &LatLng) -> String {
        let wms_params = self.get_params();
        let map_size = map.get_size();
        let point = map.lat_lng_to_container_point(latlng);

        let wms_request_parameter = WmsRequestParameter::new();
        wms_request_parameter.set_request("GetFeatureInfo".to_string());
        wms_request_parameter.set_service("WMS".to_string());
        wms_request_parameter.set_srs("EPSG:4326".to_string());
        wms_request_parameter.set_styles(wms_params.styles());
        wms_request_parameter.set_transparent(wms_params.transparent());
        wms_request_parameter.set_version(wms_params.version());
        wms_request_parameter.set_format(wms_params.format());
        wms_request_parameter.set_bbox(map.get_bounds().to_bbox_string());
        wms_request_parameter.set_height(map_size.y());
        wms_request_parameter.set_width(map_size.x());
        wms_request_parameter.set_layers(wms_params.layers());
        wms_request_parameter.set_query_layers(wms_params.layers());
        wms_request_parameter.set_info_format("geojson".to_string());

        let x = point.x().floor();
        let y = point.y().floor();

        if &wms_params.version()[..] == "1.3.0" {
            wms_request_parameter.set_i(x);
            wms_request_parameter.set_j(y);
        } else {
            wms_request_parameter.set_x(x);
            wms_request_parameter.set_y(y);
        }
        self.url()
            + &crate::Util::get_param_string_url_uppercase(
                wms_request_parameter.into(),
                &self.url(),
                true,
            )
    }
}

create_object_with_properties!(
    (TileLayerWmsOptions, TileLayerWmsOptions, TileLayerOptions),
    (layers, layers, String),
    (styles, styles, String),
    (format, format, String),
    (transparent, transparent, bool),
    (version, version, String),
    (crs, crs, Crs),
    (uppercase, uppercase, bool)
);

impl Default for TileLayerWmsOptions {
    fn default() -> Self {
        TileLayerWmsOptions::new()
    }
}

create_object_with_properties!(
    (WmsRequestParameter, WmsRequestParameter),
    (request, request, String),
    (service, service, String),
    (srs, srs, String),
    (styles, styles, String),
    (transparent, transparent, bool),
    (version, version, String),
    (format, format, String),
    (bbox, bbox, String),
    (height, height, f64),
    (width, width, f64),
    (layers, layers, String),
    (query_layers, query_layers, String),
    (info_format, info_format, String),
    (x, x, f64),
    (y, y, f64),
    (i, i, f64),
    (j, j, f64)
);
