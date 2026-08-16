use crate::world::{Biome, Flora, Terrain, Tile, World};
use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 0.021; // very large shapes
const MOISTURE_NOISE_SCALE: f64 = 0.014; // moisture-> medium regions
const FLORA_DENSITY_SCALE: f64 = 0.045; // should vegetation exists
const FLORA_TYPE_SCALE: f64 = 0.18; // what kind

pub(crate) fn generate_world(width: usize, height: usize, seed: &str) -> World {
    let height_seed = seed_to_u32(seed, 100);
    let moisture_seed = seed_to_u32(seed, 200);
    let flora_density_seed = seed_to_u32(seed, 300);
    let flora_type_seed = seed_to_u32(seed, 400);

    let height_noise = Perlin::new(height_seed);
    let moisture_noise = Perlin::new(moisture_seed);
    let flora_density_noise = Perlin::new(flora_density_seed);
    let flora_type_noise = Perlin::new(flora_type_seed);

    let mut tiles = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let raw_height = sample_noise(&height_noise, x, y, HEIGHT_NOISE_SCALE);
            let moisture = sample_noise(&moisture_noise, x, y, MOISTURE_NOISE_SCALE);
            let flora_density = sample_noise(&flora_density_noise, x, y, FLORA_DENSITY_SCALE);
            let flora_type = sample_noise(&flora_type_noise, x, y, FLORA_TYPE_SCALE);

            let shaped_height = apply_island_shape(raw_height, x, y, width, height);

            let biome = choose_biome(shaped_height);
            let terrain = generate_terrain(biome, shaped_height, moisture);
            let flora = generate_flora(terrain, moisture, flora_density, flora_type);

            tiles.push(Tile {
                height: shaped_height,
                moisture,
                biome,
                terrain,
                flora,
            });
        }
    }

    World::from_tiles(width, height, tiles)
}

fn generate_terrain(biome: Biome, height: f32, moisture: f32) -> Option<Terrain> {
    match biome {
        Biome::DeepWater | Biome::ShallowWater => None,

        Biome::Land => {
            if height > 0.56 {
                Some(Terrain::Rock)
            } else if moisture < 0.20 {
                Some(Terrain::Soil)
            } else {
                Some(Terrain::Grass)
            }
        }
    }
}

fn generate_flora(
    terrain: Option<Terrain>,
    moisture: f32,
    density: f32,
    kind: f32,
) -> Option<Flora> {
    match terrain {
        Some(Terrain::Grass) => {
            // First decide whether anything grows here.
            let threshold = if moisture > 0.7 {
                0.62
            } else if moisture > 0.5 {
                0.67
            } else {
                0.72
            };

            if density < threshold {
                return None;
            }

            // THEN independently decide what grows.
            if kind > 0.68 {
                Some(Flora::TallTree)
            } else if kind > 0.58 {
                Some(Flora::ShortTree)
            } else if kind > 0.50 {
                Some(Flora::Bush)
            } else if kind > 0.46 {
                Some(Flora::Flower)
            } else {
                None
            }
        }

        Some(Terrain::Soil) => {
            if moisture > 0.4 && density > 0.68 {
                if kind > 0.5 {
                    Some(Flora::Bush)
                } else {
                    Some(Flora::Flower)
                }
            } else {
                None
            }
        }

        Some(Terrain::Rock) | None => None,
    }
}

fn sample_noise(noise: &Perlin, x: usize, y: usize, scale: f64) -> f32 {
    let noise_x = x as f64 * scale;
    let noise_y = y as f64 * scale;

    let raw_value = noise.get([noise_x, noise_y]);

    let normalized = (raw_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

fn normalize_axis(position: usize, size: usize) -> f32 {
    if size <= 1 {
        0.0
    } else {
        position as f32 / (size - 1) as f32 * 2.0 - 1.0
    }
}

fn apply_island_shape(height_value: f32, x: usize, y: usize, width: usize, height: usize) -> f32 {
    let distance_x = normalize_axis(x, width);
    let distance_y = normalize_axis(y, height);

    let distance = (distance_x * distance_x + distance_y * distance_y)
        .sqrt()
        .min(1.0);

    let island_factor = 1.0 - distance;

    let shaped_height = height_value * 0.60 + island_factor * 0.40 - 0.08;

    shaped_height.clamp(0.0, 1.0)
}

fn choose_biome(height: f32) -> Biome {
    if height < 0.18 {
        Biome::DeepWater
    } else if height < 0.34 {
        Biome::ShallowWater
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
