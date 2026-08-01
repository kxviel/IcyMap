mod world;

use crate::world::{Biome, World};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 300;
const WORLD_HEIGHT: usize = 200;

const TILE_PIXEL: f32 = 8.0;

const CAMERA_VISIBLE_HEIGHT: f32 = 720.0;
const CAMERA_SPEED: f32 = 500.0;

const WORLD_SEED: &str = "KevinHasPotential";

fn get_biome_color(biome: Biome) -> Color {
    match biome {
        Biome::Ocean => BLUE,
        Biome::Grassland => GREEN,
        Biome::Forest => DARKGREEN,
        Biome::Desert => GOLD,
        Biome::Mountain => GRAY,
    }
}

fn draw_world(world: &World) {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            // These are world coordinates, not screen coordinates.
            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

            let color = get_biome_color(tile.biome);

            draw_rectangle(world_x, world_y, TILE_PIXEL, TILE_PIXEL, color);
        }
    }
}

fn update_camera_position(camera_position: &mut Vec2, world: &World) {
    let mut direction = Vec2::ZERO;

    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        direction.y += 1.0;
    }

    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        direction.y -= 1.0;
    }

    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        direction.x -= 1.0;
    }

    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    *camera_position += direction * CAMERA_SPEED * get_frame_time();

    let world_width = world.width as f32 * TILE_PIXEL;
    let world_height = world.height as f32 * TILE_PIXEL;

    let visible_height = CAMERA_VISIBLE_HEIGHT;
    let visible_width = visible_height * screen_width() / screen_height();

    let half_width = visible_width / 2.0;
    let half_height = visible_height / 2.0;

    camera_position.x = camera_position
        .x
        .clamp(half_width, world_width - half_width);

    camera_position.y = camera_position
        .y
        .clamp(half_height, world_height - half_height);
}

fn create_camera(camera_position: Vec2) -> Camera2D {
    let aspect_ratio = screen_width() / screen_height();

    let visible_height = CAMERA_VISIBLE_HEIGHT;
    let visible_width = visible_height * aspect_ratio;

    Camera2D::from_display_rect(Rect::new(
        camera_position.x - visible_width / 2.0,
        camera_position.y - visible_height / 2.0,
        visible_width,
        visible_height,
    ))
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        // fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world = World::new_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    let mut camera_position = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    loop {
        update_camera_position(&mut camera_position, &world);

        let camera = create_camera(camera_position);

        clear_background(BLACK);

        set_camera(&camera);
        draw_world(&world);

        set_default_camera();

        next_frame().await;
    }
}
