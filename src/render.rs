use crate::{
    TILE_PIXEL,
    world::{Biome, Flora, Terrain, Tile, World},
};
use macroquad::prelude::*;

pub(crate) fn draw_world(world: &World, camera_position: Vec2, camera_visible_height: f32) {
    draw_ground_layer(world, camera_position, camera_visible_height);
    draw_flora_layer(world, camera_position, camera_visible_height);
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
                get_tile_color(tile),
            );
        }
    }
}

fn draw_flora_layer(world: &World, camera_position: Vec2, camera_visible_height: f32) {
    let (start_x, end_x, start_y, end_y) =
        visible_tile_bounds(world, camera_position, camera_visible_height, 3);

    for y in start_y..end_y {
        for x in start_x..end_x {
            let tile = world.get_world_tile(x, y);

            if let Some(flora) = tile.flora {
                let world_x = x as f32 * TILE_PIXEL;
                let world_y = y as f32 * TILE_PIXEL;

                draw_flora(flora, world_x, world_y, TILE_PIXEL);
            }
        }
    }
}

fn draw_flora(flora: Flora, x: f32, y: f32, tile_size: f32) {
    let center_x = x + tile_size / 2.0;
    let center_y = y + tile_size / 2.0;

    match flora {
        Flora::Flower => {
            // tiny stem
            draw_line(
                center_x,
                center_y,
                center_x,
                center_y + tile_size * 0.18,
                1.0,
                Color::from_rgba(70, 100, 60, 255),
            );

            // flower head
            draw_circle(
                center_x,
                center_y,
                tile_size * 0.10,
                Color::from_rgba(220, 190, 120, 255),
            );
        }

        Flora::Bush => {
            draw_circle(
                center_x,
                center_y,
                tile_size * 0.23,
                Color::from_rgba(60, 90, 55, 255),
            );
        }

        Flora::ShortTree => {
            // trunk
            draw_rectangle(
                center_x - tile_size * 0.05,
                center_y,
                tile_size * 0.10,
                tile_size * 0.25,
                Color::from_rgba(85, 70, 50, 255),
            );

            // crown
            draw_circle(
                center_x,
                center_y - tile_size * 0.05,
                tile_size * 0.28,
                Color::from_rgba(50, 80, 50, 255),
            );
        }

        Flora::TallTree => {
            draw_rectangle(
                center_x - tile_size * 0.06,
                center_y,
                tile_size * 0.12,
                tile_size * 0.32,
                Color::from_rgba(80, 65, 45, 255),
            );

            draw_circle(
                center_x,
                center_y - tile_size * 0.12,
                tile_size * 0.34,
                Color::from_rgba(40, 70, 45, 255),
            );
        }
    }
}

fn get_tile_color(tile: &Tile) -> Color {
    match tile.biome {
        Biome::DeepWater => Color::from_rgba(35, 70, 85, 255),
        Biome::ShallowWater => Color::from_rgba(55, 105, 115, 255),

        Biome::Land => match tile.terrain {
            Some(Terrain::Grass) => Color::from_rgba(100, 125, 80, 255),
            Some(Terrain::Soil) => Color::from_rgba(125, 105, 75, 255),
            Some(Terrain::Rock) => Color::from_rgba(105, 105, 95, 255),
            None => Color::from_rgba(100, 125, 80, 255),
        },
    }
}
