mod camera;
mod generation;
mod render;
mod ui;
mod world;

use crate::camera::{clamp_camera_position, create_camera, smooth_camera, update_camera_target};
use crate::generation::generate_world;
use crate::render::draw_world;
use crate::ui::draw_hud;
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 80;
const WORLD_HEIGHT: usize = 50;

pub(crate) const TILE_PIXEL: f32 = 8.0;

pub(crate) const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 720.0;

const WORLD_SEED: &str = "Kevin'sIcyMap";

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        fullscreen: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = generate_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut camera_position = world_center;
    let mut target_camera_position = world_center;

    loop {
        update_camera_target(&mut target_camera_position, &world);

        smooth_camera(&mut camera_position, target_camera_position);

        clamp_camera_position(&mut camera_position, &world, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        let camera = create_camera(camera_position, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        clear_background(BLACK);

        set_camera(&camera);

        draw_world(&world, camera_position, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        set_default_camera();

        draw_hud();

        next_frame().await;
    }
}
