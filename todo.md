2. Too many positional f32 parameters.
   generate_world, generate_base_tiles, generate_base_tile all take 5+ same-typed scale parameters in a row (height_scale, moisture_scale, flora_density_scale, flora_type_scale, terrain_detail_scale). Nothing stops you from accidentally swapping two adjacent f32 args — the compiler can't catch it. Bundle these into a NoiseScales struct and pass one value through the call chain. Same shape of problem, lower stakes, in MapControls: five parallel (f32, String) field pairs plus five near-identical draw_scale_input calls in draw_controls. Worth collapsing into an array of a small ScaleControl struct if you touch this file again — not urgent.

3. Feature gap: no zoom. The camera is already built around a visible_height concept (create_camera, clamp_camera_position all take it as a parameter) — mouse-wheel zoom is mostly "let visible_height vary and clamp it." The one snag: update_camera_target currently reads crate::DEFAULT_CAMERA_VISIBLE_HEIGHT directly instead of accepting it as a parameter like its sibling functions do, so you'd want to fix that inconsistency first. Given how little architecture change this needs, it's a good effort/payoff ratio for a demo-facing feature.

4. Window is window_resizable: false, but the layout code is fully responsive (map_viewport_width, sidebar positioning, aspect-ratio recompute all read screen_width()/screen_height() live). Either you're planning to enable resizing later (fine, leave it), or the responsive machinery is dead weight right now — worth a conscious call either way.

5. More Biomes
