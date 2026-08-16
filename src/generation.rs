use crate::world::{Biome, Flora, Terrain, Tile, World};
use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 0.021; // large land shapes
const MOISTURE_NOISE_SCALE: f64 = 0.014; // large moisture regions
const FLORA_DENSITY_SCALE: f64 = 0.063; // where vegetation grows
const FLORA_TYPE_SCALE: f64 = 0.14; // what vegetation grows
const TERRAIN_DETAIL_SCALE: f64 = 0.14; // roughens terrain boundaries

// ==================================================
// Noise generators
// ==================================================

struct WorldNoise {
    height: Perlin,
    moisture: Perlin,
    flora_density: Perlin,
    flora_type: Perlin,
    terrain_detail: Perlin,
}

impl WorldNoise {
    fn new(seed: &str) -> Self {
        Self {
            height: Perlin::new(seed_to_u32(seed, 100)),
            moisture: Perlin::new(seed_to_u32(seed, 200)),
            flora_density: Perlin::new(seed_to_u32(seed, 300)),
            flora_type: Perlin::new(seed_to_u32(seed, 400)),
            terrain_detail: Perlin::new(seed_to_u32(seed, 500)),
        }
    }
}

// Flora is generated after terrain/beaches,
// so these values are temporarily stored.
struct FloraSample {
    density: f32,
    kind: f32,
}

// ==================================================
// World generation
// ==================================================

pub(crate) fn generate_world(width: usize, height: usize, seed: &str) -> World {
    let noise = WorldNoise::new(seed);

    let (mut tiles, flora_samples) = generate_base_tiles(width, height, &noise);

    apply_beaches(&mut tiles, width, height);
    apply_flora(&mut tiles, &flora_samples);

    World::from_tiles(width, height, tiles)
}

// ==================================================
// Base terrain generation
// ==================================================

fn generate_base_tiles(
    width: usize,
    height: usize,
    noise: &WorldNoise,
) -> (Vec<Tile>, Vec<FloraSample>) {
    let mut tiles = Vec::with_capacity(width * height);
    let mut flora_samples = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let (tile, flora_sample) = generate_base_tile(x, y, width, height, noise);

            tiles.push(tile);
            flora_samples.push(flora_sample);
        }
    }

    (tiles, flora_samples)
}

fn generate_base_tile(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    noise: &WorldNoise,
) -> (Tile, FloraSample) {
    let raw_height = sample_fbm(&noise.height, x, y, HEIGHT_NOISE_SCALE, 5, 0.5, 2.0);

    let moisture = sample_fbm(&noise.moisture, x, y, MOISTURE_NOISE_SCALE, 3, 0.5, 2.0);

    let flora_density = sample_fbm(&noise.flora_density, x, y, FLORA_DENSITY_SCALE, 3, 0.5, 2.0);

    let flora_type = sample_fbm(&noise.flora_type, x, y, FLORA_TYPE_SCALE, 1, 0.5, 2.0);

    let terrain_detail = sample_fbm(
        &noise.terrain_detail,
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

    let tile = Tile {
        height: shaped_height,
        moisture,
        biome,
        terrain,
        flora: None,
    };

    let flora_sample = FloraSample {
        density: flora_density,
        kind: flora_type,
    };

    (tile, flora_sample)
}

// ==================================================
// Terrain
// ==================================================

fn generate_terrain(biome: Biome, height: f32, moisture: f32, detail: f32) -> Option<Terrain> {
    match biome {
        Biome::DeepWater | Biome::ShallowWater => None,

        Biome::Land => {
            let rock_threshold = 0.58 + (detail - 0.5) * 0.08;

            let soil_threshold = 0.40 + (detail - 0.5) * 0.06;

            if height > rock_threshold {
                Some(Terrain::Rock)
            } else if moisture < soil_threshold {
                Some(Terrain::Soil)
            } else {
                Some(Terrain::Grass)
            }
        }
    }
}

// ==================================================
// Beaches
// ==================================================

fn apply_beaches(tiles: &mut [Tile], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;

            let can_be_beach = tiles[index].biome == Biome::Land
                && matches!(
                    tiles[index].terrain,
                    Some(Terrain::Grass) | Some(Terrain::Soil)
                );

            if !can_be_beach {
                continue;
            }

            if has_shallow_water_neighbor(tiles, x, y, width, height) {
                tiles[index].terrain = Some(Terrain::Sand);
            }
        }
    }
}

fn has_shallow_water_neighbor(
    tiles: &[Tile],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            // Skip the tile itself.
            if offset_x == 0 && offset_y == 0 {
                continue;
            }

            let neighbor_x = x as isize + offset_x;
            let neighbor_y = y as isize + offset_y;

            // Ignore positions outside the world.
            if neighbor_x < 0
                || neighbor_y < 0
                || neighbor_x >= width as isize
                || neighbor_y >= height as isize
            {
                continue;
            }

            let neighbor_index = neighbor_y as usize * width + neighbor_x as usize;

            if tiles[neighbor_index].biome == Biome::ShallowWater {
                return true;
            }
        }
    }

    false
}

// ==================================================
// Flora
// ==================================================

fn apply_flora(tiles: &mut [Tile], flora_samples: &[FloraSample]) {
    debug_assert_eq!(tiles.len(), flora_samples.len());

    for (tile, sample) in tiles.iter_mut().zip(flora_samples.iter()) {
        tile.flora = generate_flora(tile.terrain, tile.moisture, sample.density, sample.kind);
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

// ==================================================
// Noise
// ==================================================

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
    let mut max_value = 0.0;

    for _ in 0..octaves {
        let noise_x = x as f64 * base_scale * frequency;

        let noise_y = y as f64 * base_scale * frequency;

        total += noise.get([noise_x, noise_y]) * amplitude;
        max_value += amplitude;

        amplitude *= persistence;
        frequency *= lacunarity;
    }

    let normalized = (total / max_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

// ==================================================
// Island shape
// ==================================================

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

// ==================================================
// Biomes
// ==================================================

fn choose_biome(height: f32) -> Biome {
    if height < 0.18 {
        Biome::DeepWater
    } else if height < 0.34 {
        Biome::ShallowWater
    } else {
        Biome::Land
    }
}

// ==================================================
// Seed hashing
// ==================================================

fn seed_to_u32(seed: &str, salt: u32) -> u32 {
    let mut hash: u32 = 2_166_136_261;

    for byte in seed.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16_777_619);
    }

    hash.wrapping_add(salt)
}
