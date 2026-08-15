use crate::world::{Biome, Tile, World};
use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 0.015;
const MOISTURE_NOISE_SCALE: f64 = 0.070;

pub(crate) fn generate_world(width: usize, height: usize, seed: &str) -> World {
    let height_seed = seed_to_u32(seed, 100);

    let moisture_seed = seed_to_u32(seed, 200);

    let height_noise = Perlin::new(height_seed);

    let moisture_noise = Perlin::new(moisture_seed);

    let mut tiles = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let raw_height = sample_noise(&height_noise, x, y, HEIGHT_NOISE_SCALE);

            let moisture = sample_noise(&moisture_noise, x, y, MOISTURE_NOISE_SCALE);

            let shaped_height = apply_island_shape(raw_height, x, y, width, height);

            let biome = choose_biome(shaped_height, moisture);

            tiles.push(Tile {
                height: shaped_height,
                moisture,
                biome,
            });
        }
    }

    World::from_tiles(width, height, tiles)
}

fn sample_noise(noise: &Perlin, x: usize, y: usize, scale: f64) -> f32 {
    let noise_x = x as f64 * scale;
    let noise_y = y as f64 * scale;

    let raw_value = noise.get([noise_x, noise_y]);

    let normalized = (raw_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

fn apply_island_shape(height_value: f32, x: usize, y: usize, width: usize, height: usize) -> f32 {
    let center_x = width as f32 / 2.0;

    let center_y = height as f32 / 2.0;

    let distance_x = (x as f32 - center_x).abs() / center_x;

    let distance_y = (y as f32 - center_y).abs() / center_y;

    let distance_from_center = distance_x.max(distance_y);

    let island_factor = 1.0 - distance_from_center;

    let shaped_height = height_value * 0.75 + island_factor * 0.25;

    shaped_height.clamp(0.0, 1.0)
}

fn choose_biome(height: f32, moisture: f32) -> Biome {
    if height < 0.18 {
        Biome::DeepWater
    } else if height < 0.34 {
        Biome::ShallowWater
    } else if moisture > 0.62 {
        Biome::Forest
    } else {
        Biome::Land
    }
}
fn seed_to_u32(seed: &str, salt: u32) -> u32 {
    let mut hash: u32 = 2_166_136_261;

    for byte in seed.as_bytes() {
        hash ^= *byte as u32;

        hash = hash.wrapping_mul(16_777_619);
    }

    hash.wrapping_add(salt)
}
