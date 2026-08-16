use crate::world::{Biome, Flora, Terrain, Tile, World};
use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 0.021; // very large shapes
const MOISTURE_NOISE_SCALE: f64 = 0.014; // moisture-> medium regions
const FLORA_DENSITY_SCALE: f64 = 0.063; // should vegetation exists
const FLORA_TYPE_SCALE: f64 = 0.14; // what kind
const TERRAIN_DETAIL_SCALE: f64 = 0.14; // slightly disturb terrain boundaries

fn sample_fbm(
    noise: &Perlin,
    x: usize,
    y: usize,
    base_scale: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> f32 {
    debug_assert!(octaves > 0);
    debug_assert!(persistence > 0.0);
    debug_assert!(lacunarity > 0.0);

    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;

    // We need to track the maximum possible value to normalize the result correctly
    let mut max_value = 0.0;

    for _ in 0..octaves {
        let noise_x = x as f64 * base_scale * frequency;
        let noise_y = y as f64 * base_scale * frequency;

        // The noise.get function returns values roughly between -1.0 and 1.0
        total += noise.get([noise_x, noise_y]) * amplitude;
        max_value += amplitude;

        amplitude *= persistence;
        frequency *= lacunarity;
    }

    // Normalize the accumulated total back to a 0.0 - 1.0 range
    let normalized = (total / max_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

pub(crate) fn generate_world(width: usize, height: usize, seed: &str) -> World {
    let height_noise = Perlin::new(seed_to_u32(seed, 100));
    let moisture_noise = Perlin::new(seed_to_u32(seed, 200));
    let flora_density_noise = Perlin::new(seed_to_u32(seed, 300));
    let flora_type_noise = Perlin::new(seed_to_u32(seed, 500));
    let terrain_detail_noise = Perlin::new(seed_to_u32(seed, 400));

    let mut tiles = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let raw_height = sample_fbm(&height_noise, x, y, HEIGHT_NOISE_SCALE, 5, 0.5, 2.0);
            let moisture = sample_fbm(&moisture_noise, x, y, MOISTURE_NOISE_SCALE, 3, 0.5, 2.0);
            let flora_density =
                sample_fbm(&flora_density_noise, x, y, FLORA_DENSITY_SCALE, 3, 0.5, 2.0);
            let flora_type = sample_fbm(&flora_type_noise, x, y, FLORA_TYPE_SCALE, 1, 0.5, 2.0);
            let terrain_detail = sample_fbm(
                &terrain_detail_noise,
                x,
                y,
                TERRAIN_DETAIL_SCALE,
                2,
                0.5,
                2.0,
            );

            let shaped_height = apply_island_shape(raw_height, x, y, width, height);

            let biome = choose_biome(shaped_height);
            let terrain = generate_terrain(biome, shaped_height, moisture, terrain_detail);
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

fn generate_terrain(biome: Biome, height: f32, moisture: f32, detail: f32) -> Option<Terrain> {
    match biome {
        Biome::DeepWater | Biome::ShallowWater => None,

        Biome::Land => {
            let rock_threshold = 0.58 + (detail - 0.5) * 0.08;

            if height < 0.365 {
                Some(Terrain::Sand)
            } else if height > rock_threshold {
                Some(Terrain::Rock)
            } else if moisture < 0.40 {
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
            let threshold = if moisture > 0.68 {
                0.52
            } else if moisture > 0.52 {
                0.57
            } else {
                0.62
            };

            if density < threshold {
                return None;
            }

            if kind > 0.68 {
                Some(Flora::TallTree)
            } else if kind > 0.58 {
                Some(Flora::ShortTree)
            } else if kind > 0.50 {
                Some(Flora::Bush)
            } else if kind > 0.44 {
                Some(Flora::Flower)
            } else {
                None
            }
        }

        Some(Terrain::Soil) => {
            if moisture > 0.32 && density > 0.58 {
                if kind > 0.62 {
                    Some(Flora::Bush)
                } else if kind > 0.48 {
                    Some(Flora::Flower)
                } else {
                    None
                }
            } else {
                None
            }
        }

        Some(Terrain::Sand) | Some(Terrain::Rock) | None => None,
    }
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
