use crate::{
    TILE_PIXEL,
    assets::TerrainTextures,
    world::{Biome, World},
};
use macroquad::prelude::*;

pub(crate) fn draw_world(
    world: &World,
    textures: &TerrainTextures,
    camera_position: Vec2,
    camera_visible_height: f32,
) {
    draw_ground_layer(world, textures, camera_position, camera_visible_height);

    draw_tree_layer(world, textures, camera_position, camera_visible_height);
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

fn visible_tile_bounds(
    world: &World,
    camera_position: Vec2,
    camera_visible_height: f32,
    padding: usize,
) -> (usize, usize, usize, usize) {
    let aspect_ratio = screen_width() / screen_height().max(1.0);

    let camera_visible_width = camera_visible_height * aspect_ratio;

    let left = camera_position.x - camera_visible_width / 2.0;

    let right = camera_position.x + camera_visible_width / 2.0;

    let top = camera_position.y - camera_visible_height / 2.0;

    let bottom = camera_position.y + camera_visible_height / 2.0;

    let padding = padding as isize;

    let start_x =
        ((left / TILE_PIXEL).floor() as isize - padding).clamp(0, world.width as isize) as usize;

    let end_x =
        ((right / TILE_PIXEL).ceil() as isize + padding).clamp(0, world.width as isize) as usize;

    let start_y =
        ((top / TILE_PIXEL).floor() as isize - padding).clamp(0, world.height as isize) as usize;

    let end_y =
        ((bottom / TILE_PIXEL).ceil() as isize + padding).clamp(0, world.height as isize) as usize;

    (start_x, end_x, start_y, end_y)
}

fn draw_ground_layer(
    world: &World,
    textures: &TerrainTextures,
    camera_position: Vec2,
    camera_visible_height: f32,
) {
    let (start_x, end_x, start_y, end_y) =
        visible_tile_bounds(world, camera_position, camera_visible_height, 3);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = world.get_world_tile(x, y);

            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

            let hash = tile_hash(x, y);

            // Select one of the four land variants.
            // The same tile always gets the same variant.
            let land_index = ((hash >> 16) as usize) % textures.lands.len();

            let land_texture = &textures.lands[land_index];

            let (texture, allow_vertical_flip) = match tile.biome {
                Biome::Forest | Biome::Grassland => (land_texture, true),

                Biome::Desert => (&textures.desert, true),

                Biome::Ocean => (&textures.ocean, true),

                Biome::Mountain => (&textures.mountain, false),
            };

            draw_terrain_texture(texture, world_x, world_y, x, y, allow_vertical_flip);
        }
    }
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

fn draw_tree_layer(
    world: &World,
    textures: &TerrainTextures,
    camera_position: Vec2,
    camera_visible_height: f32,
) {
    let (start_x, end_x, start_y, end_y) =
        visible_tile_bounds(world, camera_position, camera_visible_height, 6);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = world.get_world_tile(x, y);

            if tile.biome != Biome::Forest {
                continue;
            }

            let hash = tile_hash(x, y);

            let forest_neighbors = forest_neighbor_count(world, x, y);

            let tree_density = match forest_neighbors {
                0..=2 => 20,
                3..=4 => 40,
                5..=6 => 60,
                _ => 75,
            };

            if hash % 100 >= tree_density {
                continue;
            }

            let tree_index = ((hash >> 24) as usize) % textures.trees.len();

            let tree_texture = &textures.trees[tree_index];

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

                    // Required by the current tree assets.
                    flip_y: true,

                    ..Default::default()
                },
            );
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

    world.get_world_tile(x as usize, y as usize).biome == Biome::Forest
}
