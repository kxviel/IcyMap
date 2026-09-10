# IcyMaps

A small island generator in Rust and Macroquad. The same seed and settings give
the same map.

```sh
cargo run --release
```

- **WASD / arrows:** move
- **Mouse wheel:** zoom over the map
- **Home:** reset the view
- **Hover:** inspect a tile
- **Enter / Regenerate Map:** apply the seed and scale settings
- **Escape / click the map:** leave the inputs

Smaller noise scales make broader patterns. Type a value or use `+` / `-` to
change it by 0.001. Values are clamped and rounded when you regenerate; invalid
input falls back to the last valid value.

The map is cached in a texture so zooming out stays fast.

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
