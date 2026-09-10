use crate::{
    noise::{NoiseScales, WorldNoise, fbm},
    world::{Biome, Flora, Terrain, Tile, World},
};

pub const DEEP_WATER_MAX_HEIGHT: f32 = 0.18;
pub const SHALLOW_WATER_MAX_HEIGHT: f32 = 0.34;

pub const ROCK_BASE_HEIGHT: f32 = 0.58;
pub const ROCK_DETAIL_RANGE: f32 = 0.08;
pub const MOUNTAIN_MIN_HEIGHT: f32 = 0.58;
pub const SNOW_BASE_HEIGHT: f32 = 0.62;

pub const SOIL_BASE_HEIGHT: f32 = 0.40;
pub const SOIL_DETAIL_RANGE: f32 = 0.06;

struct FloraSample {
    density: f32,
    kind: f32,
}

pub fn generate_world(width: usize, height: usize, seed: &str, scales: &NoiseScales) -> World {
    let noise = WorldNoise::new(seed);

    let mut tiles = Vec::with_capacity(width * height);
    let mut flora_samples = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let (tile, flora_sample) = sample_tile(x, y, width, height, &noise, scales);

            tiles.push(tile);
            flora_samples.push(flora_sample);
        }
    }

    apply_beaches(&mut tiles, width, height);
    for (tile, sample) in tiles.iter_mut().zip(flora_samples) {
        tile.flora = generate_flora(tile.terrain, tile.moisture, sample.density, sample.kind);
    }
    World::from_tiles(width, height, tiles)
}

fn sample_tile(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    noise: &WorldNoise,
    scales: &NoiseScales,
) -> (Tile, FloraSample) {
    let raw_height = fbm(&noise.height, x, y, scales.height, 5);
    let moisture = fbm(&noise.moisture, x, y, scales.moisture, 3);
    let flora_density = fbm(&noise.flora_density, x, y, scales.flora_density, 3);
    let flora_type = fbm(&noise.flora_type, x, y, scales.flora_type, 1);
    let terrain_detail = fbm(&noise.terrain_detail, x, y, scales.terrain_detail, 2);
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

fn choose_biome(height: f32) -> Biome {
    if height < DEEP_WATER_MAX_HEIGHT {
        Biome::DeepWater
    } else if height < SHALLOW_WATER_MAX_HEIGHT {
        Biome::ShallowWater
    } else if height > MOUNTAIN_MIN_HEIGHT {
        Biome::Mountain
    } else {
        Biome::Land
    }
}

fn generate_terrain(biome: Biome, height: f32, moisture: f32, detail: f32) -> Option<Terrain> {
    match biome {
        Biome::DeepWater | Biome::ShallowWater => None,

        Biome::Mountain => {
            let snow_line = SNOW_BASE_HEIGHT + (detail - 0.5) * 0.06;

            if height > snow_line {
                Some(Terrain::Snow)
            } else {
                Some(Terrain::Rock)
            }
        }

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

fn apply_beaches(tiles: &mut [Tile], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;

            let can_be_beach = tiles[index].biome == Biome::Land
                && matches!(tiles[index].terrain, Some(Terrain::Grassy | Terrain::Soil));

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
    for ny in y.saturating_sub(1)..=(y + 1).min(height - 1) {
        for nx in x.saturating_sub(1)..=(x + 1).min(width - 1) {
            if (nx != x || ny != y) && tiles[ny * width + nx].biome == Biome::ShallowWater {
                return true;
            }
        }
    }

    false
}

fn generate_flora(
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

        Some(Terrain::Sand) | Some(Terrain::Rock) | Some(Terrain::Snow) | None => None,
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

fn normalize_axis(position: usize, size: usize) -> f32 {
    if size <= 1 {
        0.0
    } else {
        position as f32 / (size - 1) as f32 * 2.0 - 1.0
    }
}
