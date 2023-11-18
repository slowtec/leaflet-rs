use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

use crate::{object_constructor, Map};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen (extends = Object , js_name = LocateOptions)]
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub type LocateOptions;

    #[wasm_bindgen(method, js_name = locate)]
    pub fn locate(this: &Map) -> Map;

    /// Tries to locate the user using the Geolocation API, firing a
    /// locationfound event with location data on success or a `locationerror`
    /// event on failure, and optionally sets the map view to the user's
    /// location with respect to detection accuracy (or to the world view if
    /// geolocation failed). Note that, if your page doesn't use HTTPS, this
    /// method will fail in modern browsers (Chrome 50 and newer)
    /// See `LocateOptions` for more details.
    #[wasm_bindgen(method, js_name = locate)]
    pub fn locate_with_options(this: &Map, options: &LocateOptions) -> Map;

    /// Stops watching location previously initiated by map.locate({watch: true})
    /// and aborts resetting the map view if map.locate was called with {setView: true}.
    #[wasm_bindgen(method, js_name = stopLocate)]
    pub fn stop_locate(this: &Map) -> Map;
}

impl LocateOptions {
    object_constructor!();

    /// If true, starts continuous watching of location changes (instead of detecting it once)
    /// using W3C watchPosition method. You can later stop watching using map.stopLocate() method.
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-watch)
    pub fn watch(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(self.as_ref(), &JsValue::from("watch"), &JsValue::from(val));
        let _ = r;
        self
    }

    /// If true, automatically sets the map view to the user location with respect to
    /// detection accuracy, or to world view if geolocation failed.
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-setView)
    pub fn set_view(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("setView"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    /// The maximum zoom for automatic view setting when using setView option.
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-maxZoom)
    pub fn max_zoom(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("maxZoom"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    /// Number of milliseconds to wait for a response from geolocation before firing a `locationerror` event.
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-timeout)
    pub fn timeout(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("timeout"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    /// Maximum age of detected location. If less than this amount of milliseconds passed since
    /// last geolocation response, locate will return a cached location.
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-maximumAge)
    pub fn maximum_age(&mut self, val: f64) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("maximumAge"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }

    /// Enables high accuracy, see [description in the W3C spec](https://w3c.github.io/geolocation-api/#enablehighaccuracy-member).
    ///
    /// [Leaflet Documentation](https://leafletjs.com/reference.html#locate-options-enableHighAccuracy)
    pub fn enable_high_accuracy(&mut self, val: bool) -> &mut Self {
        let r = Reflect::set(
            self.as_ref(),
            &JsValue::from("enableHighAccuracy"),
            &JsValue::from(val),
        );
        let _ = r;
        self
    }
}

impl Default for LocateOptions {
    fn default() -> Self {
        LocateOptions::new()
    }
}
