mod assets;
mod camera;
mod generation;
mod render;
mod ui;
mod world;

use crate::assets::TerrainTextures;
use crate::camera::{
    ZoomLevel, clamp_camera_position, create_camera, smooth_camera, update_camera_target,
    update_zoom_target,
};
use crate::generation::generate_world;
use crate::render::draw_world;
use crate::ui::draw_hud;
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 300;
const WORLD_HEIGHT: usize = 200;

pub(crate) const TILE_PIXEL: f32 = 8.0;

pub(crate) const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 720.0;
pub(crate) const MIN_CAMERA_VISIBLE_HEIGHT: f32 = 120.0;

const WORLD_SEED: &str = "KevinHasPotential";

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = generate_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    let textures = TerrainTextures::load().await;

    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut zoom_level = ZoomLevel::Default;

    let mut camera_position = world_center;
    let mut target_camera_position = world_center;

    let mut camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;

    let mut target_camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;

    loop {
        update_zoom_target(
            &mut zoom_level,
            &mut target_camera_visible_height,
            &mut target_camera_position,
            &world,
        );

        update_camera_target(
            &mut target_camera_position,
            target_camera_visible_height,
            &world,
        );

        smooth_camera(
            &mut camera_position,
            &mut camera_visible_height,
            target_camera_position,
            target_camera_visible_height,
        );

        clamp_camera_position(&mut camera_position, &world, camera_visible_height);

        let camera = create_camera(camera_position, camera_visible_height);

        clear_background(BLACK);

        set_camera(&camera);

        draw_world(&world, &textures, camera_position, camera_visible_height);

        set_default_camera();

        draw_hud(
            zoom_level,
            camera_visible_height,
            DEFAULT_CAMERA_VISIBLE_HEIGHT,
        );

        next_frame().await;
    }
}
