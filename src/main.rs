use macroquad::prelude::*;

const WORLD_WIDTH: usize = 25;
const WORLD_HEIGHT: usize = 25;
const TILE_PIXEL: f32 = 25.0;

#[derive(Clone, Copy)]
enum Biome {
    Ocean,
    Grassland,
    Forest,
    Desert,
    Mountain,
}

struct Tile {
    tile_height: f32,
    biome: Biome,
}

struct World {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl World {
    fn new(width: usize, height: usize) -> World {
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let biome = generate_test_biome(x, y, width, height);

                let tile = Tile {
                    tile_height: 0.0,
                    biome,
                };

                tiles.push(tile);
            }
        }

        World {
            width,
            height,
            tiles,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        let index = y * self.width + x;
        &self.tiles[index]
    }
}

fn generate_test_biome(x: usize, y: usize, width: usize, height: usize) -> Biome {
    if y < 4 {
        Biome::Ocean
    } else if x < 5 {
        Biome::Desert
    } else if x > width - 6 {
        Biome::Mountain
    } else if y > height - 8 {
        Biome::Forest
    } else {
        Biome::Grassland
    }
}

fn get_biome_color(biome: Biome) -> Color {
    match biome {
        Biome::Ocean => BLUE,
        Biome::Grassland => GREEN,
        Biome::Forest => DARKGREEN,
        Biome::Desert => BROWN,
        Biome::Mountain => GRAY,
    }
}

fn draw_world(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_tile(x, y);

            let screen_x = x as f32 * TILE_PIXEL;
            let screen_y = y as f32 * TILE_PIXEL;

            let color = get_biome_color(tile.biome);

            draw_rectangle(
                screen_x,
                screen_y,
                TILE_PIXEL - 1.0,
                TILE_PIXEL - 1.0,
                color,
            );
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
    let world = World::new(WORLD_WIDTH, WORLD_HEIGHT);

    loop {
        clear_background(WHITE);

        draw_text("IcyMap - World Struct", 20.0, 40.0, 32.0, BLACK);

        draw_world(&world);

        next_frame().await;
    }
}
