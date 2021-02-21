# leaflet-rs

A [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
wrapper for
[Leaflet.js](https://leafletjs.com/)

## Usage

```toml
[dependencies]
leaflet = "0.1"
```

## Example

```rust
use leaflet::{Circle, LatLng, LatLngBounds, Map, Polygon, Polyline, Rectangle, TileLayer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CircleOptions {
    radius: f32,
}

#[derive(Serialize, Deserialize)]
struct PolylineOptions {
    color: String,
}

fn init() {
    let map = Map::new("map", &JsValue::NULL);
    map.setView(&LatLng::new(63.5, 10.5), 5.0);

    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", &JsValue::NULL).addTo(&map);

    Polyline::new_with_options(
        [LatLng::new(63.25, 11.25), LatLng::new(63.75, 11.75), LatLng::new(63.5, 12.0)]
            .iter().map(JsValue::from).collect(),
        &JsValue::from_serde(&PolylineOptions { color: "red".into() }).expect("Unable to serialize polyline options")
    ).addTo(&map);

    Polygon::new(
        [LatLng::new(63.25, 12.25), LatLng::new(63.75, 12.75), LatLng::new(63.5, 13.0)]
            .iter().map(JsValue::from).collect()
    ).addTo(&map);

    Rectangle::new(&LatLngBounds::new(&LatLng::new(63.25, 10.25), &LatLng::new(63.75, 10.75))).addTo(&map);

    Circle::new(&LatLng::new(63.25, 13.25)).addTo(&map);

    Circle::new_with_options(
        &LatLng::new(63.25, 13.35),
        &JsValue::from_serde(&CircleOptions { radius: 4000.0 }).expect("Unable to serialize circle options")
    ).addTo(&map);
}
```

## License

Copyright (c) 2020 - 2021 [slowtec GmbH](https://slowtec.de)
