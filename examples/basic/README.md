# Leaflet-rs example

## How to build
```
# Unix
RUSTC_BOOTSTRAP=1 cargo install --git https://github.com/thecoshman/http
```

```
# Windows
set RUSTC_BOOTSTRAP=1
cargo install --git https://github.com/thecoshman/http
```

```
cargo install wasm-pack 
# in current directory, so: examples/basic
wasm-pack build --target web
http
```

Then open http://localhost:8000
