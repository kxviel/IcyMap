use crate::{
    TILE_PIXEL,
    world::{Biome, World},
};
use macroquad::prelude::*;

pub(crate) fn draw_world(world: &World, camera_position: Vec2, camera_visible_height: f32) {
    draw_ground_layer(world, camera_position, camera_visible_height);
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

fn draw_ground_layer(world: &World, camera_position: Vec2, camera_visible_height: f32) {
    let (start_x, end_x, start_y, end_y) =
        visible_tile_bounds(world, camera_position, camera_visible_height, 3);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = world.get_world_tile(x, y);

            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

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

fn get_biome_color(biome: Biome) -> Color {
    match biome {
        Biome::ShallowWater => Color::from_rgba(39, 217, 245, 204),
        Biome::DeepWater => Color::from_rgba(0, 64, 97, 204),
        Biome::Forest => Color::from_rgba(46, 92, 55, 204),
        Biome::Land => Color::from_rgba(90, 160, 70, 204),
    }
}
