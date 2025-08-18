use js_sys::Array;
use leaflet::{
    Canvas, CanvasOptions, Circle, CircleOptions, LatLng, LatLngBounds, Map, MapOptions, Polygon,
    Polyline, PolylineOptions, Rectangle, Svg, SvgOptions, TileLayer,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Running SVG and Canvas Renderer with Panes example in Rust.".into());

    // Example 1: Create SVG map with two different panes and renderers
    create_svg_map_with_multiple_panes()?;

    // Example 2: Create Canvas map with two different panes and renderers
    create_canvas_map_with_multiple_panes()?;

    Ok(())
}

fn create_svg_map_with_multiple_panes() -> Result<(), JsValue> {
    console::log_1(&"Creating SVG map with multiple panes and renderers...".into());

    // Create the map with default options first
    let map_options = MapOptions::default();
    let map = Map::new("svg-map", &map_options);
    map.set_view(&LatLng::new(51.505, -0.09), 13.0);

    // Add a tile layer
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);

    // Create the panes first
    map.create_pane_by_name("pane1");
    map.create_pane_by_name("pane2");

    // Create first SVG renderer for pane1 with red styling
    let svg_options_1 = SvgOptions::default();
    svg_options_1.set_pane("pane1".to_string());
    svg_options_1.set_padding(0.05);
    svg_options_1.set_tolerance(3.0);
    svg_options_1.set_stroke(true);
    svg_options_1.set_color("#ff0000".to_string());
    svg_options_1.set_weight(4.0);
    svg_options_1.set_opacity(0.8);
    svg_options_1.set_fill(true);
    svg_options_1.set_fill_color("#ffcccc".to_string());
    svg_options_1.set_fill_opacity(0.4);

    let svg_renderer_1 = Svg::with_options(&svg_options_1);

    // Create second SVG renderer for pane2 with blue styling
    let svg_options_2 = SvgOptions::default();
    svg_options_2.set_pane("pane2".to_string());
    svg_options_2.set_padding(0.1);
    svg_options_2.set_tolerance(5.0);
    svg_options_2.set_stroke(true);
    svg_options_2.set_color("#0000ff".to_string());
    svg_options_2.set_weight(2.0);
    svg_options_2.set_opacity(0.9);
    svg_options_2.set_fill(true);
    svg_options_2.set_fill_color("#ccccff".to_string());
    svg_options_2.set_fill_opacity(0.3);

    let svg_renderer_2 = Svg::with_options(&svg_options_2);

    // Add vector shapes using the first renderer (red styling)
    add_shapes_with_renderer(&map, svg_renderer_1.into(), "red");

    // Add different vector shapes using the second renderer (blue styling)
    add_shapes_with_renderer(&map, svg_renderer_2.into(), "blue");

    Ok(())
}

fn create_canvas_map_with_multiple_panes() -> Result<(), JsValue> {
    console::log_1(&"Creating Canvas map with multiple panes and renderers...".into());

    // Create the map with default options first
    let map_options = MapOptions::default();
    let map = Map::new("canvas-map", &map_options);
    map.set_view(&LatLng::new(51.505, -0.09), 13.0);

    // Add a tile layer
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);

    // Create the panes first
    map.create_pane_by_name("canvas-pane1");
    map.create_pane_by_name("canvas-pane2");

    // Create first Canvas renderer for pane1 with green styling
    let canvas_options_1 = CanvasOptions::default();
    canvas_options_1.set_pane("canvas-pane1".to_string());
    canvas_options_1.set_padding(0.03);
    canvas_options_1.set_tolerance(2.0);
    canvas_options_1.set_stroke(true);
    canvas_options_1.set_color("#00aa00".to_string());
    canvas_options_1.set_weight(3.0);
    canvas_options_1.set_opacity(0.8);
    canvas_options_1.set_fill(true);
    canvas_options_1.set_fill_color("#ccffcc".to_string());
    canvas_options_1.set_fill_opacity(0.4);

    let canvas_renderer_1 = Canvas::with_options(&canvas_options_1);

    // Create second Canvas renderer for pane2 with purple styling
    let canvas_options_2 = CanvasOptions::default();
    canvas_options_2.set_pane("canvas-pane2".to_string());
    canvas_options_2.set_padding(0.08);
    canvas_options_2.set_tolerance(4.0);
    canvas_options_2.set_stroke(true);
    canvas_options_2.set_color("#aa00aa".to_string());
    canvas_options_2.set_weight(5.0);
    canvas_options_2.set_opacity(0.9);
    canvas_options_2.set_fill(true);
    canvas_options_2.set_fill_color("#ffccff".to_string());
    canvas_options_2.set_fill_opacity(0.3);

    let canvas_renderer_2 = Canvas::with_options(&canvas_options_2);

    // Add vector shapes using the first renderer (green styling)
    add_shapes_with_renderer(&map, canvas_renderer_1.into(), "green");

    // Add different vector shapes using the second renderer (purple styling)
    add_shapes_with_renderer(&map, canvas_renderer_2.into(), "purple");

    Ok(())
}

fn add_shapes_with_renderer(map: &Map, renderer: JsValue, color_scheme: &str) {
    let offset = match color_scheme {
        "red" => (0.0, 0.0),
        "blue" => (0.01, 0.01),
        "green" => (0.0, 0.0),
        "purple" => (-0.01, -0.01),
        _ => (0.0, 0.0),
    };

    // Add a polyline with renderer-specific styling
    let polyline_points = [
        LatLng::new(51.5 + offset.0, -0.09 + offset.1),
        LatLng::new(51.51 + offset.0, -0.08 + offset.1),
        LatLng::new(51.52 + offset.0, -0.07 + offset.1),
    ];
    let polyline_array = polyline_points.iter().map(JsValue::from).collect::<Array>();

    let polyline_options = PolylineOptions::default();
    polyline_options.set_renderer(renderer.clone());
    match color_scheme {
        "red" => {
            polyline_options.set_color("#cc0000".to_string());
            polyline_options.set_weight(6.0);
        }
        "blue" => {
            polyline_options.set_color("#0000cc".to_string());
            polyline_options.set_weight(3.0);
        }
        "green" => {
            polyline_options.set_color("#008800".to_string());
            polyline_options.set_weight(4.0);
        }
        "purple" => {
            polyline_options.set_color("#880088".to_string());
            polyline_options.set_weight(7.0);
        }
        _ => {}
    }

    Polyline::new_with_options(&polyline_array, &polyline_options).add_to(map);

    // Add a polygon with renderer-specific styling
    let polygon_points = [
        LatLng::new(51.515 + offset.0, -0.09 + offset.1),
        LatLng::new(51.52 + offset.0, -0.08 + offset.1),
        LatLng::new(51.52 + offset.0, -0.06 + offset.1),
        LatLng::new(51.515 + offset.0, -0.07 + offset.1),
    ];
    let polygon_array = polygon_points.iter().map(JsValue::from).collect::<Array>();

    let polygon_options = PolylineOptions::default();
    polygon_options.set_renderer(renderer.clone());
    match color_scheme {
        "red" => {
            polygon_options.set_color("#990000".to_string());
            polygon_options.set_fill_color("#ffaaaa".to_string());
        }
        "blue" => {
            polygon_options.set_color("#000099".to_string());
            polygon_options.set_fill_color("#aaaaff".to_string());
        }
        "green" => {
            polygon_options.set_color("#006600".to_string());
            polygon_options.set_fill_color("#aaffaa".to_string());
        }
        "purple" => {
            polygon_options.set_color("#660066".to_string());
            polygon_options.set_fill_color("#ffaaff".to_string());
        }
        _ => {}
    }

    Polygon::new_with_options(&polygon_array, &polygon_options).add_to(map);

    // Add a rectangle with renderer-specific styling
    let rect_offset = match color_scheme {
        "blue" | "purple" => 0.02,
        _ => 0.0,
    };

    let bounds = LatLngBounds::new(
        &LatLng::new(51.49 + offset.0 + rect_offset, -0.08 + offset.1),
        &LatLng::new(51.505 + offset.0 + rect_offset, -0.06 + offset.1),
    );

    let rectangle_options = PolylineOptions::default();
    rectangle_options.set_renderer(renderer.clone());
    match color_scheme {
        "red" => {
            rectangle_options.set_color("#aa0000".to_string());
            rectangle_options.set_fill_color("#ffdddd".to_string());
        }
        "blue" => {
            rectangle_options.set_color("#0000aa".to_string());
            rectangle_options.set_fill_color("#ddddff".to_string());
        }
        "green" => {
            rectangle_options.set_color("#00aa00".to_string());
            rectangle_options.set_fill_color("#ddffdd".to_string());
        }
        "purple" => {
            rectangle_options.set_color("#aa00aa".to_string());
            rectangle_options.set_fill_color("#ffddff".to_string());
        }
        _ => {}
    }

    Rectangle::new_with_options(&bounds, &rectangle_options).add_to(map);

    // Add a circle with renderer-specific styling
    let circle_offset = match color_scheme {
        "blue" => (0.005, -0.02),
        "purple" => (-0.005, 0.02),
        _ => (0.0, 0.0),
    };

    let circle_options = CircleOptions::default();
    circle_options.set_renderer(renderer);
    circle_options.set_radius(150.0);
    match color_scheme {
        "red" => {
            circle_options.set_color("#dd0000".to_string());
            circle_options.set_fill_color("#ffe0e0".to_string());
        }
        "blue" => {
            circle_options.set_color("#0000dd".to_string());
            circle_options.set_fill_color("#e0e0ff".to_string());
        }
        "green" => {
            circle_options.set_color("#00dd00".to_string());
            circle_options.set_fill_color("#e0ffe0".to_string());
        }
        "purple" => {
            circle_options.set_color("#dd00dd".to_string());
            circle_options.set_fill_color("#ffe0ff".to_string());
        }
        _ => {}
    }

    Circle::new_with_options(
        &LatLng::new(
            51.508 + offset.0 + circle_offset.0,
            -0.11 + offset.1 + circle_offset.1,
        ),
        &circle_options,
    )
    .add_to(map);
}
