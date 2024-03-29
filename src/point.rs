use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Point;

    #[wasm_bindgen(constructor, js_namespace = L)]
    pub fn new(x: f64, y: f64) -> Point;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Point) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Point) -> f64;

    #[wasm_bindgen(method)]
    pub fn add(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn subtract(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method, js_name = multiplyBy)]
    pub fn multiply_by(this: &Point, scalar: f64) -> Point;

    #[wasm_bindgen(method, js_name = divideBy)]
    pub fn divide_by(this: &Point, scalar: f64) -> Point;

    #[wasm_bindgen(method, js_name = scaleBy)]
    pub fn scale_by(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method, js_name = unscaleByTo)]
    pub fn unscale_by_to(this: &Point, other: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn round(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn floor(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn ceil(this: &Point) -> Point;

    #[wasm_bindgen(method)]
    pub fn trunc(this: &Point) -> bool;

    #[wasm_bindgen(method)]
    pub fn equals(this: &Point, other: &Point) -> bool;

    #[wasm_bindgen(method)]
    pub fn contains(this: &Point, other: &Point) -> f64;

    #[wasm_bindgen(method, js_name = distanceTo)]
    pub fn distance_to(this: &Point, other: &Point) -> f64;
}

impl From<(u32, u32)> for Point {
    fn from((x, y): (u32, u32)) -> Point {
        Point::new(f64::from(x), f64::from(y))
    }
}

impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point::new(x, y)
    }
}
