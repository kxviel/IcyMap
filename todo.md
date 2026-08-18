3. Feature gap: no zoom. The camera is already built around a visible_height concept (create_camera, clamp_camera_position all take it as a parameter) — mouse-wheel zoom is mostly "let visible_height vary and clamp it." The one snag: update_camera_target currently reads crate::DEFAULT_CAMERA_VISIBLE_HEIGHT directly instead of accepting it as a parameter like its sibling functions do, so you'd want to fix that inconsistency first. Given how little architecture change this needs, it's a good effort/payoff ratio for a demo-facing feature.

4. More Biomes
