use js_sys::Object;
use url::{ParseError, Url};
use wasm_bindgen::prelude::*;

use crate::{Crs, LatLng, Map, TileLayer, TileLayerOptions, create_object_with_properties};

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

/// Construct a request URL for a WMS service.
///
/// # Defaults
/// The default implementation creates a request with the following adjusted parameters:
///
/// - Request: `GetFeatureInfo`
/// - Service: `WMS`
/// - SRS: `EPSG:3426`
/// - Info format: `geojson`
pub struct WmsRequestBuilder {
    params: WmsRequestParameter,
}

impl WmsRequestBuilder {
    /// Sets the service parameter.
    #[must_use]
    pub fn with_service(self, service: &str) -> Self {
        self.params.set_service(service.to_string());
        self
    }

    /// Sets the request parameter.
    #[must_use]
    pub fn with_request(self, request: &str) -> Self {
        self.params.set_request(request.to_string());
        self
    }

    /// Sets the srs parameter.
    #[must_use]
    pub fn with_srs(self, srs: &str) -> Self {
        self.params.set_srs(srs.to_string());
        self
    }

    /// Sets the info format parameter.
    #[must_use]
    pub fn with_info_format(self, info_format: &str) -> Self {
        self.params.set_info_format(info_format.to_string());
        self
    }

    /// Creates the [Url] for the request with the parameters set previously.
    pub fn build(
        &self,
        map: &Map,
        layer: &TileLayerWms,
        latlng: &LatLng,
    ) -> Result<Url, ParseError> {
        let wms_params = layer.get_params();
        let map_size = map.get_size();
        let point = map.lat_lng_to_container_point(latlng);

        self.params.set_styles(wms_params.styles());
        self.params.set_transparent(wms_params.transparent());
        self.params.set_version(wms_params.version());
        self.params.set_format(wms_params.format());
        self.params.set_bbox(map.get_bounds().to_bbox_string());
        self.params.set_height(map_size.y());
        self.params.set_width(map_size.x());
        self.params.set_layers(wms_params.layers());
        self.params.set_query_layers(wms_params.layers());

        let x = point.x().floor();
        let y = point.y().floor();

        if &wms_params.version()[..] == "1.3.0" {
            self.params.set_i(x);
            self.params.set_j(y);
        } else {
            self.params.set_x(x);
            self.params.set_y(y);
        }

        Url::parse(
            &(layer.url()
                + &crate::Util::get_param_string_url_uppercase(
                    self.params.clone().into(),
                    &layer.url(),
                    true,
                )),
        )
    }
}

impl Default for WmsRequestBuilder {
    fn default() -> Self {
        let params = WmsRequestParameter::new();
        params.set_request("GetFeatureInfo".to_string());
        params.set_service("WMS".to_string());
        params.set_srs("EPSG:4326".to_string());
        params.set_info_format("geojson".to_string());
        Self { params }
    }
}
