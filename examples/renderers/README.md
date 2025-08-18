# SVG and Canvas Renderer Example

This example demonstrates how to use both the SVG and Canvas renderers with leaflet-rs, which are equivalent to using `L.svg()` and `L.canvas()` in JavaScript Leaflet.

## Overview

Leaflet provides two main renderers for vector graphics. Both inherit from the base `Renderer` class, which itself extends `Layer`:

- `Renderer` (extends `Layer`) - Base class for all renderers
  - `Svg` (extends `Renderer`) - SVG-based renderer
  - `Canvas` (extends `Renderer`) - Canvas-based renderer

### SVG Renderer
The SVG renderer uses Scalable Vector Graphics (SVG) and is particularly useful when you need:
- Scalable vector graphics that maintain quality at all zoom levels
- Individual DOM elements for each shape (for styling/interaction)
- Custom styling and animations
- Accessibility features that come with SVG elements

### Canvas Renderer
The Canvas renderer uses HTML5 Canvas and is particularly useful when you need:
- Better performance with many vector elements
- Consistent rendering across browsers
- Lower memory usage for complex scenes
- Faster rendering for animations

## Usage

### Basic SVG Renderer

Create a map with the default SVG renderer:

```rust
use leaflet::{Map, MapOptions, Svg};

// Create SVG renderer with default options
let svg_renderer = Svg::new();

// Set up map options with the renderer
let map_options = MapOptions::default();
map_options.set_renderer(svg_renderer.into());

// Create the map
let map = Map::new("map", &map_options);
```

This is equivalent to the JavaScript:
```javascript
var map = L.map('map', {
    renderer: L.svg()
});
```

### Custom SVG Renderer Options

Create an SVG renderer with custom options:

```rust
use leaflet::{Map, MapOptions, Svg, SvgOptions};

// Create SVG renderer with custom options
let svg_options = SvgOptions::default();
svg_options.set_padding(0.1);
svg_options.set_tolerance(5.0);
svg_options.set_stroke(true);
svg_options.set_color("#ff0000".to_string());
svg_options.set_weight(3.0);
svg_options.set_opacity(0.8);
svg_options.set_fill(true);
svg_options.set_fill_color("#00ff00".to_string());
svg_options.set_fill_opacity(0.3);

let svg_renderer = Svg::with_options(&svg_options);

// Set up map options with the custom renderer
let map_options = MapOptions::default();
map_options.set_renderer(svg_renderer.into());

// Create the map
let map = Map::new("map", &map_options);
```

### Canvas Renderer

Create a map with the Canvas renderer:

```rust
use leaflet::{Map, MapOptions, Canvas, CanvasOptions};

// Create Canvas renderer with default options
let canvas_renderer = Canvas::new();

// Set up map options with the renderer
let map_options = MapOptions::default();
map_options.set_renderer(canvas_renderer.into());

// Create the map
let map = Map::new("map", &map_options);
```

This is equivalent to the JavaScript:
```javascript
var map = L.map('map', {
    renderer: L.canvas()
});
```

## Inheritance Hierarchy

In leaflet-rs, the renderer types follow this inheritance hierarchy:

```
Evented
  └── Layer
      └── Renderer
          ├── Svg
          └── Canvas
```

This matches the JavaScript Leaflet inheritance where:
- `L.Renderer` extends `L.Layer`
- `L.SVG` extends `L.Renderer`  
- `L.Canvas` extends `L.Renderer`

## Available Options

Both `SvgOptions` and `CanvasOptions` structs support the following properties:

### Renderer Options
- `padding`: Extra padding around the rendered area
- `tolerance`: Tolerance for simplifying polylines and polygons

### Style Options
- `stroke`: Whether to draw stroke along the path
- `color`: Stroke color
- `weight`: Stroke width in pixels
- `opacity`: Stroke opacity
- `line_cap`: Line cap style ('round', 'butt', 'square')
- `line_join`: Line join style ('round', 'bevel', 'miter')
- `dash_array`: Stroke dash pattern
- `dash_offset`: Distance into the dash pattern to start the dash
- `fill`: Whether to fill the path with color
- `fill_color`: Fill color
- `fill_opacity`: Fill opacity
- `fill_rule`: Fill rule ('nonzero', 'evenodd')

### Interaction Options
- `interactive`: Whether the layer is interactive
- `bubbling_mouse_events`: Whether mouse events bubble
- `class_name`: Custom CSS class name

## Running the Example

1. Build the WebAssembly module:
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

2. Serve the files using a local web server:
   ```bash
   python -m http.server 8000
   # or
   npx serve .
   ```

3. Open your browser to `http://localhost:8000` to see the example in action.

The example shows three maps:
- Map 1: Default SVG renderer
- Map 2: Custom SVG renderer with styling options
- Map 3: Canvas renderer for performance comparison

## Choosing Between SVG and Canvas

**Choose SVG when:**
- You need scalable vector graphics
- You want individual DOM elements for each shape (for styling/interaction)
- You're working with a moderate number of vector elements
- You need accessibility features
- You want to apply CSS styles to individual elements

**Choose Canvas when:**
- You have many vector elements (better performance)
- You don't need individual DOM elements
- You want consistent rendering across browsers
- Memory usage is a concern
- You need faster rendering for animations