mod world;

use crate::world::{Biome, World};
use macroquad::prelude::*;

const WORLD_WIDTH: usize = 120;
const WORLD_HEIGHT: usize = 80;
const TILE_PIXEL: f32 = 8.0;

const WORLD_SEED: &str = "icy-map-001";

// Smaller = larger continents.
// Bigger = small terrain patches.
const HEIGHT_NOISE_SCALE: f64 = 0.045;
const MOISTURE_NOISE_SCALE: f64 = 0.070;

fn get_biome_color(biome: Biome) -> Color {
    match biome {
        Biome::Ocean => BLUE,
        Biome::Grassland => GREEN,
        Biome::Forest => DARKGREEN,
        Biome::Desert => GOLD,
        Biome::Mountain => GRAY,
    }
}

fn draw_world(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            let screen_x = x as f32 * TILE_PIXEL;
            let screen_y = y as f32 * TILE_PIXEL + 60.0;

            let color = get_biome_color(tile.biome);

            draw_rectangle(screen_x, screen_y, TILE_PIXEL, TILE_PIXEL, color);
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = World::new_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    loop {
        clear_background(WHITE);

        draw_world(&world);

        next_frame().await;
    }
}
