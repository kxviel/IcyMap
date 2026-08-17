# IcyMaps — Implementation Notes

## Project scope

IcyMaps is a deterministic procedural map generator and viewer built with Rust and Macroquad. It generates one complete tile world from a text seed, renders the terrain and flora, supports smooth camera movement, and exposes tile data through a hover inspector.

## Current functionality

- Seeded Perlin/fBm world generation
- Island shaping
- Deep and shallow water
- Sand, soil, grass, and rock terrain
- Moisture-driven terrain variation
- Procedural flowers, bushes, and trees
- Layered terrain and flora rendering
- Visible-tile culling
- Smooth keyboard camera movement and world-bound clamping
- HUD with FPS display
- Hovered-tile inspector for position, height, moisture, biome, terrain, and flora

## Source structure

```text
src/
├── main.rs        # application setup and frame loop
├── camera.rs      # camera movement, smoothing, clamping, and mouse conversion
├── generation.rs  # seeded noise, island shaping, terrain, beaches, and flora
├── render.rs      # visible tile bounds and map layers
├── ui.rs          # HUD and tile inspector
└── world.rs       # map data types and tile storage
```

## Determinism and preservation rules

- The same seed and generation parameters must produce the same map.
- Generation changes should remain explicit and testable.
- Existing world dimensions, terrain thresholds, island shaping, and visual style should stay stable unless a change is intentional.
- Rendering and UI must consume the generated world without mutating it.

## Future direction

IcyMaps can later expose selected generation parameters through a small control panel and regenerate the map without restarting. Useful controls include the seed, Perlin/fBm scales, octave settings, and explicit presets such as large islands, small islands, archipelagos, or one dominant island. Each mode should remain deterministic and display its active values.
