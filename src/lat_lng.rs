/*
 * Copyright (c) HeadlessStudio  2023.
 */

use std::ops::{Add, AddAssign, Div, Sub};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Default, Clone)]
    pub type LatLng;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(lat: f64, lng: f64) -> LatLng;

    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LatLng) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LatLng) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LatLng, value: f64) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LatLng, value: f64) -> f64;

    #[wasm_bindgen(method, js_name = distanceTo)]
    pub fn distance_to(this: &LatLng, otherLatLng: &LatLng) -> f64;

}

#[allow(clippy::from_over_into)]
impl Into<LatLng> for (f64, f64) {
    fn into(self) -> LatLng {
        LatLng::new(self.0, self.1)
    }
}

#[allow(clippy::from_over_into)]
impl Into<LatLng> for [f64; 2] {
    fn into(self) -> LatLng {
        LatLng::new(self[0], self[1])
    }
}

impl Add<LatLng> for LatLng {
    type Output = LatLng;

    fn add(self, rhs: LatLng) -> Self::Output {
        LatLng::new(self.lat() + rhs.lat(), self.lng() + rhs.lng())
    }
}

impl Add<LatLng> for &LatLng {
    type Output = LatLng;

    fn add(self, rhs: LatLng) -> Self::Output {
        LatLng::new(self.lat() + rhs.lat(), self.lng() + rhs.lng())
    }
}

impl Sub<LatLng> for LatLng {
    type Output = LatLng;

    fn sub(self, rhs: LatLng) -> Self::Output {
        LatLng::new(self.lat() - rhs.lat(), self.lng() - rhs.lng())
    }
}

impl Sub<&LatLng> for LatLng {
    type Output = LatLng;

    fn sub(self, rhs: &LatLng) -> Self::Output {
        LatLng::new(self.lat() - rhs.lat(), self.lng() - rhs.lng())
    }
}

impl Sub<LatLng> for &LatLng {
    type Output = LatLng;

    fn sub(self, rhs: LatLng) -> Self::Output {
        LatLng::new(self.lat() - rhs.lat(), self.lng() - rhs.lng())
    }
}

impl AddAssign<LatLng> for LatLng {
    fn add_assign(&mut self, rhs: LatLng) {
        self.set_lat(self.lat() + rhs.lat());
        self.set_lng(self.lng() + rhs.lng());
    }
}

impl Div<f64> for LatLng {
    type Output = LatLng;

    fn div(self, rhs: f64) -> Self::Output {
        LatLng::new(self.lat() / rhs, self.lng() / rhs)
    }
}

impl Div<f64> for &LatLng {
    type Output = LatLng;

    fn div(self, rhs: f64) -> Self::Output {
        LatLng::new(self.lat() / rhs, self.lng() / rhs)
    }
}
