use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 0.015; // low fragmentation -> small value
const MOISTURE_NOISE_SCALE: f64 = 0.070;

#[derive(Clone, Copy)]
pub(crate) enum Biome {
    Ocean,
    Grassland,
    Forest,
    Desert,
    Mountain,
}

pub struct Tile {
    height: f32,
    moisture: f32,
    pub(crate) biome: Biome,
}

pub struct World {
    pub(crate) width: usize,
    pub(crate) height: usize,
    tiles: Vec<Tile>,
}

fn seed_to_u32(seed: &str, salt: u32) -> u32 {
    let mut hash: u32 = 2166136261;

    for byte in seed.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16777619);
    }

    hash.wrapping_add(salt)
}

fn sample_noise(noise: &Perlin, x: usize, y: usize, scale: f64) -> f32 {
    let nx = x as f64 * scale;
    let ny = y as f64 * scale;

    let raw_value = noise.get([nx, ny]);

    // Perlin noise usually returns values around -1.0 to 1.0.
    // We convert that into 0.0 to 1.0 because biome rules are easier that way.
    let normalized = (raw_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

fn apply_island_shape(height_value: f32, x: usize, y: usize, width: usize, height: usize) -> f32 {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;

    let dx = (x as f32 - center_x).abs() / center_x;
    let dy = (y as f32 - center_y).abs() / center_y;

    let distance_from_center = dx.max(dy);

    // 1.0 near center, 0.0 near edges.
    let island_factor = 1.0 - distance_from_center;

    // Blend noise height with island shape.
    let shaped = height_value * 0.75 + island_factor * 0.25;

    shaped.clamp(0.0, 1.0)
}

fn choose_biome(height: f32, moisture: f32) -> Biome {
    if height < 0.34 {
        Biome::Ocean
    } else if height > 0.78 {
        Biome::Mountain
    } else if moisture < 0.20 {
        Biome::Desert
    } else if moisture > 0.62 {
        Biome::Forest
    } else {
        Biome::Grassland
    }
}

impl World {
    pub fn new_world(width: usize, height: usize, seed: &str) -> World {
        let height_seed = seed_to_u32(seed, 100);
        let moisture_seed = seed_to_u32(seed, 200);

        let height_noise = Perlin::new(height_seed);
        let moisture_noise = Perlin::new(moisture_seed);

        let mut tiles: Vec<Tile> = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let height_value = sample_noise(&height_noise, x, y, HEIGHT_NOISE_SCALE);

                let moisture_value = sample_noise(&moisture_noise, x, y, MOISTURE_NOISE_SCALE);

                let shaped_height = apply_island_shape(height_value, x, y, width, height);

                let biome = choose_biome(shaped_height, moisture_value);

                let tile = Tile {
                    height: shaped_height,
                    moisture: moisture_value,
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

    pub fn get_world_tile(&self, x: usize, y: usize) -> &Tile {
        let index = y * self.width + x;
        &self.tiles[index]
    }
}
