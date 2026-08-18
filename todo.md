3. Feature gap: no zoom. The camera is already built around a visible_height concept (create_camera, clamp_camera_position all take it as a parameter) — mouse-wheel zoom is mostly "let visible_height vary and clamp it." The one snag: update_camera_target currently reads crate::DEFAULT_CAMERA_VISIBLE_HEIGHT directly instead of accepting it as a parameter like its sibling functions do, so you'd want to fix that inconsistency first. Given how little architecture change this needs, it's a good effort/payoff ratio for a demo-facing feature.

4. Window is window_resizable: false, but the layout code is fully responsive (map_viewport_width, sidebar positioning, aspect-ratio recompute all read screen_width()/screen_height() live). Either you're planning to enable resizing later (fine, leave it), or the responsive machinery is dead weight right now — worth a conscious call either way.

5. More Biomes
