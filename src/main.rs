mod camera;
mod generation;
mod noise;
mod render;
mod ui;
mod world;

use crate::camera::{
    clamp_camera_position, create_camera, mouse_world_position, smooth_camera, update_camera_target,
};
use crate::generation::generate_world;
use crate::noise::NoiseScales;
use crate::render::draw_world;
use crate::ui::{MapControls, draw_controls, draw_hud, mouse_is_over_sidebar};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 240;
const WORLD_HEIGHT: usize = 150;

pub(crate) const TILE_PIXEL: f32 = 8.0;

const ZOOM_STEP: f32 = 50.0;
const MIN_CAMERA_VISIBLE_HEIGHT: f32 = 180.0;
const MAX_CAMERA_VISIBLE_HEIGHT: f32 = 1200.0;
const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 630.0;

const MAX_CAMERA_DELTA_SECONDS: f32 = 0.05;

fn load_icon(path: &str) -> Option<miniquad::conf::Icon> {
    use image::imageops::FilterType;

    let image = image::open(path)
        .expect("Failed to load window icon")
        .to_rgba8();

    let small = image::imageops::resize(&image, 16, 16, FilterType::Lanczos3);
    let medium = image::imageops::resize(&image, 32, 32, FilterType::Lanczos3);
    let big = image::imageops::resize(&image, 64, 64, FilterType::Lanczos3);

    Some(miniquad::conf::Icon {
        small: small.into_raw().try_into().ok()?,
        medium: medium.into_raw().try_into().ok()?,
        big: big.into_raw().try_into().ok()?,
    })
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMaps".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        fullscreen: false,
        high_dpi: false,
        icon: load_icon("assets/icymaps_icon_1024.png"),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut controls = MapControls::new();
    let scales = controls.noise_scales();

    let mut world = generate_world(WORLD_WIDTH, WORLD_HEIGHT, &controls.seed, &scales);

    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut camera_position = world_center;
    let mut target_camera_position = world_center;

    let mut camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;

    loop {
        // Camera

        let camera_delta_time = get_frame_time().min(MAX_CAMERA_DELTA_SECONDS);

        let (_, wheel_y) = mouse_wheel();

        if wheel_y > 0.0 {
            camera_visible_height = (camera_visible_height - ZOOM_STEP)
                .clamp(MIN_CAMERA_VISIBLE_HEIGHT, MAX_CAMERA_VISIBLE_HEIGHT);
        } else if wheel_y < 0.0 {
            camera_visible_height = (camera_visible_height + ZOOM_STEP)
                .clamp(MIN_CAMERA_VISIBLE_HEIGHT, MAX_CAMERA_VISIBLE_HEIGHT);
        }

        update_camera_target(
            &mut target_camera_position,
            &world,
            camera_delta_time,
            camera_visible_height,
        );

        smooth_camera(
            &mut camera_position,
            target_camera_position,
            camera_delta_time,
        );

        clamp_camera_position(&mut camera_position, &world, camera_visible_height);

        let camera = create_camera(camera_position, camera_visible_height);

        // Tile Inspector

        let mouse_world = mouse_world_position(&camera);

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
                world.get_world_tile(tile_x as usize, tile_y as usize),
            ))
        } else {
            None
        };

        // Render

        clear_background(BLACK);

        set_camera(&camera);

        draw_world(&world, camera_position, camera_visible_height);

        set_default_camera();

        draw_hud(hovered_tile);

        let regenerate = draw_controls(&mut controls);

        if regenerate {
            let scales = NoiseScales {
                height: controls.height_scale,
                moisture: controls.moisture_scale,
                flora_density: controls.flora_density_scale,
                flora_type: controls.flora_type_scale,
                terrain_detail: controls.terrain_detail_scale,
            };

            world = generate_world(WORLD_WIDTH, WORLD_HEIGHT, &controls.seed, &scales);
        }

        next_frame().await;
    }
}
