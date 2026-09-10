mod camera;
mod generation;
mod noise;
mod render;
mod ui;
mod world;

use crate::camera::{clamp_camera_position, create_camera, smooth_camera, update_camera_target};
use crate::generation::generate_world;
use crate::render::MapRenderer;
use crate::ui::{MapControls, draw_controls, mouse_is_over_sidebar};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 240;
const WORLD_HEIGHT: usize = 150;

pub const TILE_PIXEL: f32 = 8.0;

const ZOOM_STEP: f32 = 50.0;
const MIN_VIEW_HEIGHT: f32 = 180.0;
const MAX_VIEW_HEIGHT: f32 = 1200.0;
const DEFAULT_VIEW_HEIGHT: f32 = 630.0;

const MAX_FRAME_TIME: f32 = 0.05;

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMaps".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        high_dpi: true,
        icon: Some(miniquad::conf::Icon {
            small: *include_bytes!("../assets/icymaps_icon_16.rgba"),
            medium: *include_bytes!("../assets/icymaps_icon_32.rgba"),
            big: *include_bytes!("../assets/icymaps_icon_64.rgba"),
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    request_new_screen_size(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    let mut controls = MapControls::new();
    let mut world = generate_world(
        WORLD_WIDTH,
        WORLD_HEIGHT,
        &controls.seed,
        &controls.noise_scales(),
    );

    let mut renderer = MapRenderer::new(&world);

    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut camera_position = world_center;
    let mut target_position = world_center;

    let mut visible_height = DEFAULT_VIEW_HEIGHT;

    loop {
        controls.update_focus();
        let dt = get_frame_time().min(MAX_FRAME_TIME);
        let (_, wheel_y) = mouse_wheel();

        if !mouse_is_over_sidebar() && wheel_y != 0.0 {
            visible_height = (visible_height - wheel_y.signum() * ZOOM_STEP)
                .clamp(MIN_VIEW_HEIGHT, MAX_VIEW_HEIGHT);
        }

        if !controls.focused {
            if is_key_pressed(KeyCode::Home) {
                camera_position = world_center;
                target_position = world_center;
                visible_height = DEFAULT_VIEW_HEIGHT;
            }

            update_camera_target(&mut target_position, dt);
        }

        clamp_camera_position(&mut target_position, &world, visible_height);
        smooth_camera(&mut camera_position, target_position, dt);
        clamp_camera_position(&mut camera_position, &world, visible_height);

        let camera = create_camera(camera_position, visible_height);

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_world = camera.screen_to_world(vec2(mouse_x, mouse_y));
        let tile_x = (mouse_world.x / TILE_PIXEL).floor() as isize;
        let tile_y = (mouse_world.y / TILE_PIXEL).floor() as isize;

        let hovered_tile = if !mouse_is_over_sidebar()
            && tile_x >= 0
            && tile_y >= 0
            && tile_x < world.width as isize
            && tile_y < world.height as isize
        {
            Some((
                tile_x as usize,
                tile_y as usize,
                world.tile(tile_x as usize, tile_y as usize),
            ))
        } else {
            None
        };

        clear_background(Color::from_hex(0x0a1014));
        set_camera(&camera);
        renderer.draw();

        if let Some((x, y, _)) = hovered_tile {
            draw_rectangle_lines(
                x as f32 * TILE_PIXEL,
                y as f32 * TILE_PIXEL,
                TILE_PIXEL,
                TILE_PIXEL,
                visible_height / screen_height().max(1.0),
                Color::new(1.0, 1.0, 1.0, 0.7),
            );
        }

        set_default_camera();

        if draw_controls(&mut controls, hovered_tile) {
            world = generate_world(
                WORLD_WIDTH,
                WORLD_HEIGHT,
                &controls.seed,
                &controls.noise_scales(),
            );
            renderer = MapRenderer::new(&world);
        }

        next_frame().await;
    }
}
