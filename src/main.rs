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
    land: Texture2D,
    mountain: Texture2D,
    ocean: Texture2D,
    desert: Texture2D,
    trees: [Texture2D; 4],
}

impl TerrainTextures {
    async fn load() -> Self {
        let tree_1 = load_texture("assets/objects/tree-sprite-16x32-001.png")
            .await
            .expect("Failed to load tree sprite 001");
        let tree_2 = load_texture("assets/objects/tree-sprite-16x32-002.png")
            .await
            .expect("Failed to load tree sprite 002");
        let tree_3 = load_texture("assets/objects/tree-sprite-16x32-003.png")
            .await
            .expect("Failed to load tree sprite 003");
        let tree_4 = load_texture("assets/objects/tree-sprite-16x32-004.png")
            .await
            .expect("Failed to load tree sprite 004");
        let trees = [tree_1, tree_2, tree_3, tree_4];

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

        for tree in &trees {
            tree.set_filter(FilterMode::Nearest);
        }

        land.set_filter(FilterMode::Nearest);
        mountain.set_filter(FilterMode::Nearest);
        ocean.set_filter(FilterMode::Nearest);
        desert.set_filter(FilterMode::Nearest);

        Self {
            land,
            mountain,
            ocean,
            desert,
            trees,
        }
    }
}

fn tile_hash(x: usize, y: usize) -> u32 {
    (x as u32).wrapping_mul(73_856_093) ^ (y as u32).wrapping_mul(19_349_663)
}

fn tile_variation(x: usize, y: usize) -> (bool, bool) {
    let hash = tile_hash(x, y);

    let flip_x = hash & 1 != 0;
    let flip_y = hash & 2 != 0;

    (flip_x, flip_y)
}

fn draw_terrain_texture(
    texture: &Texture2D,
    world_x: f32,
    world_y: f32,
    tile_x: usize,
    tile_y: usize,
    allow_vertical_flip: bool,
) {
    let (flip_x, generated_flip_y) = tile_variation(tile_x, tile_y);

    draw_texture_ex(
        texture,
        world_x,
        world_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(TILE_PIXEL, TILE_PIXEL)),
            flip_x,
            flip_y: allow_vertical_flip && generated_flip_y,
            ..Default::default()
        },
    );
}

fn draw_ground_layer(world: &World, textures: &TerrainTextures) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

            let (texture, allow_vertical_flip) = match tile.biome {
                // Forest uses grass as its ground layer.
                // Trees are drawn separately afterward.
                Biome::Forest => (&textures.land, true),
                Biome::Grassland => (&textures.land, true),
                Biome::Desert => (&textures.desert, true),
                Biome::Ocean => (&textures.ocean, true),
                Biome::Mountain => (&textures.mountain, false),
            };

            draw_terrain_texture(texture, world_x, world_y, x, y, allow_vertical_flip);
        }
    }
}

fn forest_neighbor_count(world: &World, x: usize, y: usize) -> u32 {
    let mut count = 0;

    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            if offset_x == 0 && offset_y == 0 {
                continue;
            }

            if is_forest_tile(world, x as isize + offset_x, y as isize + offset_y) {
                count += 1;
            }
        }
    }

    count
}

fn is_forest_tile(world: &World, x: isize, y: isize) -> bool {
    if x < 0 || y < 0 || x >= world.width as isize || y >= world.height as isize {
        return false;
    }

    matches!(
        world.get_world_tile(x as usize, y as usize).biome,
        Biome::Forest
    )
}

fn draw_tree_layer(world: &World, textures: &TerrainTextures) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            if !matches!(tile.biome, Biome::Forest) {
                continue;
            }

            let hash = tile_hash(x, y);
            let tree_index = ((hash >> 24) as usize) % textures.trees.len();
            let tree_texture = &textures.trees[tree_index];

            let forest_neighbors = forest_neighbor_count(world, x, y);
            let tree_density = match forest_neighbors {
                0..=2 => 20,
                3..=4 => 40,
                5..=6 => 55,
                _ => 65,
            };

            if hash % 100 >= tree_density {
                continue;
            }

            let offset_x = (((hash >> 8) & 255) as f32 / 255.0 - 0.5) * 4.0;
            let offset_y = (((hash >> 16) & 255) as f32 / 255.0 - 0.5) * 1.0;

            let tree_size = vec2(12.0, 24.0);
            let tile_world_x = x as f32 * TILE_PIXEL;
            let tile_world_y = y as f32 * TILE_PIXEL;

            let tree_x = tile_world_x + TILE_PIXEL / 2.0 - tree_size.x / 2.0 + offset_x;

            let tree_y = tile_world_y + TILE_PIXEL - tree_size.y + offset_y;

            draw_texture_ex(
                tree_texture,
                tree_x,
                tree_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(tree_size),
                    flip_x: hash & 1 != 0,
                    flip_y: true,
                    ..Default::default()
                },
            );
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

        draw_ground_layer(&world, &textures);
        draw_tree_layer(&world, &textures);

        set_default_camera();

        next_frame().await;
    }
}
