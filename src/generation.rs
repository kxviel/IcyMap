use crate::{
    noise::{NoiseScales, WorldNoise, fbm_gen},
    world::{Biome, Flora, Terrain, Tile, World},
};

pub(crate) const DEEP_WATER_MAX_HEIGHT: f32 = 0.18;
pub(crate) const SHALLOW_WATER_MAX_HEIGHT: f32 = 0.34;

pub(crate) const ROCK_BASE_HEIGHT: f32 = 0.58;
pub(crate) const ROCK_DETAIL_RANGE: f32 = 0.08;
pub(crate) const MOUNTAIN_MIN_HEIGHT: f32 = 0.58;
pub(crate) const SNOW_BASE_HEIGHT: f32 = 0.62;

pub(crate) const SOIL_BASE_HEIGHT: f32 = 0.40;
pub(crate) const SOIL_DETAIL_RANGE: f32 = 0.06;

const VOLCANO_MIN_LAVA: f32 = 0.64;
const ROT_MIN_STRENGTH: f32 = 0.68;
const SWAMP_MIN_MOISTURE: f32 = 0.70;
const VOLCANO_LAVA_MIN_STRENGTH: f32 = 0.73;
const SWAMP_WATER_MIN_WETNESS: f32 = 0.76;

struct FloraSample {
    density: f32,
    kind: f32,
}

// Main API

pub(crate) fn generate_world(
    width: usize,
    height: usize,
    seed: &str,
    scales: &NoiseScales,
) -> World {
    let noise = WorldNoise::new(seed);

    let (mut tiles, flora_samples) = generate_base_tiles(width, height, &noise, scales);

    apply_beaches(&mut tiles, width, height);
    apply_flora(&mut tiles, &flora_samples);

    World::from_tiles(width, height, tiles)
}

// Phase 1: Base Terrain Generation

fn generate_base_tiles(
    width: usize,
    height: usize,
    noise: &WorldNoise,
    scales: &NoiseScales,
) -> (Vec<Tile>, Vec<FloraSample>) {
    let mut tiles = Vec::with_capacity(width * height);
    let mut flora_samples = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let (tile, flora_sample) = generate_base_tile(x, y, width, height, noise, scales);

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
    scales: &NoiseScales,
) -> (Tile, FloraSample) {
    let raw_height = fbm_gen(&noise.height, x, y, scales.height as f64, 5, 0.5, 2.0);
    let moisture = fbm_gen(&noise.moisture, x, y, scales.moisture as f64, 3, 0.5, 2.0);
    let flora_density = fbm_gen(
        &noise.flora_density,
        x,
        y,
        scales.flora_density as f64,
        3,
        0.5,
        2.0,
    );
    let flora_type = fbm_gen(
        &noise.flora_type,
        x,
        y,
        scales.flora_type as f64,
        1,
        0.5,
        2.0,
    );
    let terrain_detail = fbm_gen(
        &noise.terrain_detail,
        x,
        y,
        scales.terrain_detail as f64,
        2,
        0.5,
        2.0,
    );
    let lava = fbm_gen(&noise.lava, x, y, 0.025, 3, 0.5, 2.0);
    let rot = fbm_gen(&noise.rot, x, y, 0.015, 3, 0.5, 2.0);

    let shaped_height = apply_island_shape(raw_height, x, y, width, height);
    let biome = choose_biome(shaped_height, moisture, lava, rot);
    let terrain = generate_terrain(biome, shaped_height, moisture, terrain_detail, lava);

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

fn choose_biome(height: f32, moisture: f32, lava: f32, rot: f32) -> Biome {
    if height < DEEP_WATER_MAX_HEIGHT {
        Biome::DeepWater
    } else if height < SHALLOW_WATER_MAX_HEIGHT {
        Biome::ShallowWater
    } else if lava > VOLCANO_MIN_LAVA {
        Biome::Volcano
    } else if height > MOUNTAIN_MIN_HEIGHT {
        Biome::Mountain
    } else if rot > ROT_MIN_STRENGTH {
        Biome::Rot
    } else if moisture > SWAMP_MIN_MOISTURE {
        Biome::Swamp
    } else {
        Biome::Land
    }
}

fn generate_terrain(
    biome: Biome,
    height: f32,
    moisture: f32,
    detail: f32,
    lava: f32,
) -> Option<Terrain> {
    match biome {
        Biome::DeepWater | Biome::ShallowWater => None,

        Biome::Volcano => {
            let lava_strength =
                lava + (detail - 0.5) * 0.12 + (height - SHALLOW_WATER_MAX_HEIGHT) * 0.08;

            if lava_strength > VOLCANO_LAVA_MIN_STRENGTH {
                Some(Terrain::Lava)
            } else {
                Some(Terrain::Rock)
            }
        }

        Biome::Swamp => {
            let wetness = moisture + (detail - 0.5) * 0.16;

            if wetness > SWAMP_WATER_MIN_WETNESS {
                Some(Terrain::SwampWater)
            } else {
                Some(Terrain::Mud)
            }
        }

        Biome::Mountain => {
            let snow_line = SNOW_BASE_HEIGHT + (detail - 0.5) * 0.06;

            if height > snow_line {
                Some(Terrain::Snow)
            } else {
                Some(Terrain::Rock)
            }
        }

        Biome::Rot => Some(Terrain::Soil),

        Biome::Land => {
            let rock_threshold = ROCK_BASE_HEIGHT + (detail - 0.5) * ROCK_DETAIL_RANGE;

            let soil_threshold = SOIL_BASE_HEIGHT + (detail - 0.5) * SOIL_DETAIL_RANGE;

            if height > rock_threshold {
                Some(Terrain::Rock)
            } else if moisture < soil_threshold {
                Some(Terrain::Soil)
            } else {
                Some(Terrain::Grassy)
            }
        }
    }
}

// Phase 2: Beach Application

fn apply_beaches(tiles: &mut [Tile], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;

            let can_be_beach = tiles[index].biome == Biome::Land
                && matches!(
                    tiles[index].terrain,
                    Some(Terrain::Grassy) | Some(Terrain::Soil)
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
            if offset_x == 0 && offset_y == 0 {
                continue;
            }

            let neighbor_x = x as isize + offset_x;
            let neighbor_y = y as isize + offset_y;

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

// Phase 3: Flora Application

fn apply_flora(tiles: &mut [Tile], flora_samples: &[FloraSample]) {
    debug_assert_eq!(tiles.len(), flora_samples.len());

    for (tile, sample) in tiles.iter_mut().zip(flora_samples.iter()) {
        tile.flora = generate_flora(
            tile.biome,
            tile.terrain,
            tile.moisture,
            sample.density,
            sample.kind,
        );
    }
}

fn generate_flora(
    biome: Biome,
    terrain: Option<Terrain>,
    moisture: f32,
    density: f32,
    kind: f32,
) -> Option<Flora> {
    match biome {
        Biome::Rot => {
            if matches!(terrain, Some(Terrain::Soil)) && density > 0.60 {
                Some(Flora::DeadTree)
            } else {
                None
            }
        }

        Biome::Swamp => match terrain {
            Some(Terrain::Mud) if density > 0.60 => Some(Flora::MangroveTree),
            Some(Terrain::SwampWater) if density > 0.58 => Some(Flora::Reeds),
            _ => None,
        },

        Biome::Land => generate_land_flora(terrain, moisture, density, kind),

        Biome::DeepWater | Biome::ShallowWater | Biome::Mountain | Biome::Volcano => None,
    }
}

fn generate_land_flora(
    terrain: Option<Terrain>,
    moisture: f32,
    density: f32,
    kind: f32,
) -> Option<Flora> {
    match terrain {
        Some(Terrain::Grassy) => {
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

        Some(Terrain::Sand)
        | Some(Terrain::Rock)
        | Some(Terrain::Snow)
        | Some(Terrain::Lava)
        | Some(Terrain::Mud)
        | Some(Terrain::SwampWater)
        | None => None,
    }
}

// Math & Utility Helpers

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

fn normalize_axis(position: usize, size: usize) -> f32 {
    if size <= 1 {
        0.0
    } else {
        position as f32 / (size - 1) as f32 * 2.0 - 1.0
    }
}
