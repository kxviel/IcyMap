use crate::{
    TILE_PIXEL,
    generation::{
        DEEP_WATER_MAX_HEIGHT, MOUNTAIN_MIN_HEIGHT, ROCK_BASE_HEIGHT, ROCK_DETAIL_RANGE,
        SHALLOW_WATER_MAX_HEIGHT, SNOW_BASE_HEIGHT,
    },
    world::{Biome, Flora, Terrain, Tile, World},
};
use macroquad::prelude::*;

pub struct MapRenderer {
    map: RenderTarget,
}

impl MapRenderer {
    pub fn new(world: &World) -> Self {
        let width = world.width as f32 * TILE_PIXEL;
        let height = world.height as f32 * TILE_PIXEL;
        let map = render_target(width as u32, height as u32);
        map.texture.set_filter(FilterMode::Nearest);

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
        camera.render_target = Some(map.clone());

        // Rebuild only when the world changes; zooming just draws this texture.
        set_camera(&camera);
        clear_background(BLACK);
        draw_ground_layer(world);
        draw_flora_layer(world);
        set_default_camera();

        Self { map }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.map.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
    }
}

fn draw_ground_layer(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.tile(x, y);
            let position = vec2(x as f32 * TILE_PIXEL, y as f32 * TILE_PIXEL);
            let hash = tile_hash(x, y);
            let variation = (hash % 101) as f32 / 100.0 - 0.5;
            let shade = terrain_shade(world, x, y) + variation * 0.045;
            let color = tint(tile_color(tile), shade);

            draw_rectangle(position.x, position.y, TILE_PIXEL, TILE_PIXEL, color);
            draw_terrain_detail(tile, position, color, hash);

            if tile.biome == Biome::ShallowWater {
                draw_shoreline(world, x, y);
            }
        }
    }
}

fn terrain_shade(world: &World, x: usize, y: usize) -> f32 {
    if matches!(
        world.tile(x, y).biome,
        Biome::DeepWater | Biome::ShallowWater
    ) {
        return 1.0;
    }

    let left = world.tile(x.saturating_sub(1), y).height;
    let right = world.tile((x + 1).min(world.width - 1), y).height;
    let below = world.tile(x, y.saturating_sub(1)).height;
    let above = world.tile(x, (y + 1).min(world.height - 1)).height;

    // Light from the upper left, using the existing height field.
    (1.0 + (right - left + below - above) * 3.0).clamp(0.82, 1.16)
}

fn draw_terrain_detail(tile: &Tile, position: Vec2, color: Color, hash: u32) {
    let x = position.x + 1.0 + (hash % 5) as f32;
    let y = position.y + 1.0 + ((hash >> 8) % 5) as f32;

    match tile.terrain {
        Some(Terrain::Grassy) if hash.is_multiple_of(3) => {
            let grass = tint(color, 0.89);
            draw_rectangle(x, y, 1.0, 2.0, grass);
            draw_rectangle(x + 1.0, y, 1.0, 1.0, grass);
        }
        Some(Terrain::Sand | Terrain::Soil) => {
            draw_rectangle(x, y, 1.0, 1.0, tint(color, 0.91));
            draw_rectangle(
                position.x + 6.0,
                position.y + 2.0,
                1.0,
                1.0,
                tint(color, 1.06),
            );
        }
        Some(Terrain::Rock) if hash.is_multiple_of(3) => {
            draw_rectangle(x, y, 2.0, 1.0, tint(color, 0.86));
            draw_rectangle(x, y + 1.0, 2.0, 1.0, tint(color, 1.08));
        }
        Some(Terrain::Snow) if hash.is_multiple_of(4) => {
            draw_rectangle(x, y, 2.0, 1.0, tint(color, 0.95));
        }
        None if hash.is_multiple_of(11) => {
            let ripple = Color::new(0.65, 0.84, 0.87, 0.13);
            draw_rectangle(x, y, 2.0 + (hash % 2) as f32, 0.5, ripple);
        }
        _ => {}
    }
}

fn draw_shoreline(world: &World, x: usize, y: usize) {
    let px = x as f32 * TILE_PIXEL;
    let py = y as f32 * TILE_PIXEL;
    let foam = Color::from_rgba(177, 219, 200, 160);
    let edge = 0.7;

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx >= world.width as isize || ny >= world.height as isize {
            continue;
        }
        if matches!(
            world.tile(nx as usize, ny as usize).biome,
            Biome::Land | Biome::Mountain
        ) {
            let (offset_x, offset_y, width, height) = match (dx, dy) {
                (-1, 0) => (0.0, 0.0, edge, TILE_PIXEL),
                (1, 0) => (TILE_PIXEL - edge, 0.0, edge, TILE_PIXEL),
                (0, -1) => (0.0, 0.0, TILE_PIXEL, edge),
                _ => (0.0, TILE_PIXEL - edge, TILE_PIXEL, edge),
            };
            draw_rectangle(px + offset_x, py + offset_y, width, height, foam);
        }
    }
}

fn draw_flora_layer(world: &World) {
    // Draw the back rows first so plants overlap correctly.
    for y in (0..world.height).rev() {
        for x in 0..world.width {
            if let Some(flora) = world.tile(x, y).flora {
                let hash = tile_hash(x, y);
                let center = vec2(x as f32 + 0.5, y as f32 + 0.5) * TILE_PIXEL + flora_jitter(hash);
                draw_flora(flora, center, hash);
            }
        }
    }
}

fn draw_flora(flora: Flora, center: Vec2, hash: u32) {
    let x = center.x;
    let y = center.y;
    let shade = 0.94 + (hash % 13) as f32 * 0.01;
    let shadow = Color::from_rgba(15, 39, 42, 65);
    let trunk = Color::from_hex(0x5e4a38);

    match flora {
        Flora::Flower => {
            let petal = match hash % 3 {
                0 => Color::from_hex(0xeccc73),
                1 => Color::from_hex(0xdaaab3),
                _ => Color::from_hex(0xdae4c4),
            };
            draw_rectangle(x, y - 1.0, 0.6, 1.8, Color::from_hex(0x366348));
            draw_rectangle(x - 0.4, y + 0.5, 1.4, 1.0, petal);
        }
        Flora::Bush => {
            draw_ellipse(x + 0.8, y - 1.0, 2.3, 1.1, 0.0, shadow);
            draw_poly(x, y, 7, 1.9, 10.0, tint(Color::from_hex(0x30654b), shade));
            draw_poly(x - 0.5, y + 0.5, 6, 1.1, 0.0, Color::from_hex(0x54885b));
        }
        Flora::ShortTree => {
            draw_ellipse(x + 1.0, y - 1.5, 3.4, 1.6, 0.0, shadow);
            draw_rectangle(x - 0.5, y - 1.8, 1.0, 3.0, trunk);
            draw_poly(
                x,
                y + 1.0,
                8,
                2.9,
                22.5,
                tint(Color::from_hex(0x285542), shade),
            );
            draw_poly(
                x - 0.5,
                y + 1.8,
                7,
                2.0,
                10.0,
                tint(Color::from_hex(0x45774f), shade),
            );
            draw_poly(x - 1.0, y + 2.2, 6, 1.0, 0.0, Color::from_hex(0x6b9460));
        }
        Flora::TallTree => {
            draw_ellipse(x + 1.3, y - 1.7, 3.3, 1.5, 0.0, shadow);
            draw_rectangle(x - 0.5, y - 2.0, 1.0, 3.2, trunk);
            let dark = tint(Color::from_hex(0x1e463f), shade);
            let light = tint(Color::from_hex(0x33644d), shade);
            for (base, radius) in [(y - 0.8, 3.0), (y + 1.0, 2.3), (y + 2.6, 1.5)] {
                let tip = vec2(x, base + radius * 1.5);
                draw_triangle(vec2(x - radius, base), vec2(x + radius, base), tip, dark);
                draw_triangle(
                    vec2(x - radius, base),
                    vec2(x - 0.2, base + 0.5),
                    tip,
                    light,
                );
            }
        }
    }
}

fn tile_hash(x: usize, y: usize) -> u32 {
    let mut hash = (x as u32).wrapping_mul(374_761_393) ^ (y as u32).wrapping_mul(668_265_263);
    hash = (hash ^ (hash >> 13)).wrapping_mul(1_274_126_177);
    hash ^ (hash >> 16)
}

fn flora_jitter(hash: u32) -> Vec2 {
    vec2(
        (hash & 255) as f32 / 255.0 - 0.5,
        ((hash >> 8) & 255) as f32 / 255.0 - 0.5,
    ) * TILE_PIXEL
        * 0.42
}

fn tint(color: Color, factor: f32) -> Color {
    Color::new(
        (color.r * factor).clamp(0.0, 1.0),
        (color.g * factor).clamp(0.0, 1.0),
        (color.b * factor).clamp(0.0, 1.0),
        color.a,
    )
}

fn lerp_color(a: u32, b: u32, t: f32) -> Color {
    let a = Color::from_hex(a);
    let b = Color::from_hex(b);
    let t = t.clamp(0.0, 1.0);
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        1.0,
    )
}

fn tile_color(tile: &Tile) -> Color {
    let (low, high, amount) = match tile.biome {
        Biome::DeepWater => (0x132b40, 0x194053, tile.height / DEEP_WATER_MAX_HEIGHT),
        Biome::ShallowWater => (
            0x1c4e62,
            0x3e8990,
            (tile.height - DEEP_WATER_MAX_HEIGHT)
                / (SHALLOW_WATER_MAX_HEIGHT - DEEP_WATER_MAX_HEIGHT),
        ),
        Biome::Land | Biome::Mountain => match tile.terrain {
            Some(Terrain::Grassy) => (0x759458, 0x407356, (tile.moisture - 0.40) / 0.42),
            Some(Terrain::Sand) => (
                0xafa375,
                0xd2c48f,
                (tile.height - SHALLOW_WATER_MAX_HEIGHT) / 0.04,
            ),
            Some(Terrain::Soil) => (0x8b7053, 0x70624c, tile.moisture / 0.40),
            Some(Terrain::Rock) => {
                let base = if tile.biome == Biome::Mountain {
                    MOUNTAIN_MIN_HEIGHT
                } else {
                    ROCK_BASE_HEIGHT - ROCK_DETAIL_RANGE / 2.0
                };
                (
                    0x536771,
                    0x859595,
                    (tile.height - base) / (SNOW_BASE_HEIGHT - base),
                )
            }
            Some(Terrain::Snow) => (0xbed3db, 0xebf3ec, (tile.height - SNOW_BASE_HEIGHT) / 0.18),
            None => return Color::from_hex(0x578059),
        },
    };
    lerp_color(low, high, amount)
}
