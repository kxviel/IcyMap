use macroquad::prelude::*;

const WORLD_WIDTH: usize = 25;
const WORLD_HEIGHT: usize = 25;
const TILE_PIXEL: f32 = 25.0;
const WORLD_SEED: &str = "SEED1";

struct SeededRng {
    state: u64,
}

impl SeededRng {
    fn new(seed: &str) -> SeededRng {
        let mut state = hash_seed(seed);

        if state == 0 {
            state = 1;
        }

        SeededRng { state }
    }

    fn next_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;

        (self.state >> 32) as u32
    }

    fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }
}

#[derive(Clone, Copy)]
enum Biome {
    Ocean,
    Grassland,
    Forest,
    Desert,
    Mountain,
}

struct Tile {
    height: f32,
    moisture: f32,
    biome: Biome,
}

struct World {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl World {
    fn new(width: usize, height: usize, seed: &str) -> World {
        let mut rng = SeededRng::new(seed);
        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);

        for _y in 0..height {
            for _x in 0..width {
                let tile_height = rng.next_f32();
                let moisture = rng.next_f32();

                let biome = choose_biome(tile_height, moisture);

                let tile = Tile {
                    height: tile_height,
                    moisture,
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

fn hash_seed(seed: &str) -> u64 {
    let mut hash = 14695981039346656037u64;

    for byte in seed.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(1099511628211u64);
    }

    hash
}

fn choose_biome(height: f32, moisture: f32) -> Biome {
    if height < 0.30 {
        Biome::Ocean
    } else if height > 0.82 {
        Biome::Mountain
    } else if moisture < 0.18 {
        Biome::Desert
    } else if moisture > 0.62 {
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
        window_width: 1980,
        window_height: 1080,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = World::new(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    loop {
        clear_background(WHITE);

        draw_world(&world);

        next_frame().await;
    }
}
