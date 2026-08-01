mod camera;
mod world;

use crate::camera::{
    clamp_camera_position, create_camera, smooth_camera, update_camera_target, update_zoom_target,
};
use crate::world::{Biome, World};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 300;
const WORLD_HEIGHT: usize = 200;

const TILE_PIXEL: f32 = 8.0;

const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 720.0;
const MIN_CAMERA_VISIBLE_HEIGHT: f32 = 120.0;

const WORLD_SEED: &str = "KevinHasPotential";

struct TerrainTextures {
    forest: Texture2D,
    land: Texture2D,
    mountain: Texture2D,
    ocean: Texture2D,
    desert: Texture2D,
}

impl TerrainTextures {
    async fn load() -> Self {
        let forest = load_texture("assets/terrain/forest 32x32.png")
            .await
            .expect("Failed to load forest texture");
        let land = load_texture("assets/terrain/land 32x32.png")
            .await
            .expect("Failed to load land texture");
        let mountain = load_texture("assets/terrain/mountain 32x32.png")
            .await
            .expect("Failed to load mountain texture");
        let ocean = load_texture("assets/terrain/ocean 32x32.png")
            .await
            .expect("Failed to load ocean texture");
        let desert = load_texture("assets/terrain/desert 32x32.png")
            .await
            .expect("Failed to load desert texture");

        forest.set_filter(FilterMode::Nearest);
        land.set_filter(FilterMode::Nearest);
        mountain.set_filter(FilterMode::Nearest);
        ocean.set_filter(FilterMode::Nearest);
        desert.set_filter(FilterMode::Nearest);

        // Include all variables in the returned struct
        Self {
            forest,
            land,
            mountain,
            ocean,
            desert,
        }
    }
}

fn get_biome_color(biome: Biome) -> Color {
    match biome {
        Biome::Ocean => BLUE,
        Biome::Grassland => GREEN,
        Biome::Forest => DARKGREEN,
        Biome::Desert => GOLD,
        Biome::Mountain => GRAY,
    }
}

fn draw_world(world: &World, textures: &TerrainTextures) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

            match tile.biome {
                Biome::Forest => {
                    draw_texture_ex(
                        &textures.forest,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
                            ..Default::default()
                        },
                    );
                }
                Biome::Desert => {
                    draw_texture_ex(
                        &textures.desert,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
                            ..Default::default()
                        },
                    );
                }
                Biome::Grassland => {
                    draw_texture_ex(
                        &textures.land,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
                            ..Default::default()
                        },
                    );
                }
                Biome::Ocean => {
                    draw_texture_ex(
                        &textures.ocean,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
                            ..Default::default()
                        },
                    );
                }
                Biome::Mountain => {
                    draw_texture_ex(
                        &textures.mountain,
                        world_x,
                        world_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
                            ..Default::default()
                        },
                    );
                }

                _ => {
                    draw_rectangle(
                        world_x,
                        world_y,
                        TILE_PIXEL,
                        TILE_PIXEL,
                        get_biome_color(tile.biome),
                    );
                }
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = World::new_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);
    let textures = TerrainTextures::load().await;

    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut camera_position = world_center;
    let mut target_camera_position = world_center;

    let mut camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;
    let mut target_camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;

    loop {
        update_zoom_target(
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
        draw_world(&world, &textures);

        set_default_camera();

        next_frame().await;
    }
}
