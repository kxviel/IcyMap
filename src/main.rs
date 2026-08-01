mod world;

use crate::world::{Biome, World};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 300;
const WORLD_HEIGHT: usize = 200;

const TILE_PIXEL: f32 = 8.0;

const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 720.0;
const MIN_CAMERA_VISIBLE_HEIGHT: f32 = 160.0;
const CAMERA_SPEED: f32 = 500.0;
const ZOOM_FACTOR: f32 = 0.90;

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

            let world_x = x as f32 * TILE_PIXEL;
            let world_y = y as f32 * TILE_PIXEL;

            let color = get_biome_color(tile.biome);

            draw_rectangle(world_x, world_y, TILE_PIXEL, TILE_PIXEL, color);
        }
    }
}

fn create_camera(camera_position: Vec2, visible_height: f32) -> Camera2D {
    let aspect_ratio = screen_width() / screen_height();
    let visible_width = visible_height * aspect_ratio;

    Camera2D::from_display_rect(Rect::new(
        camera_position.x - visible_width / 2.0,
        camera_position.y - visible_height / 2.0,
        visible_width,
        visible_height,
    ))
}

fn get_maximum_visible_height(world: &World) -> f32 {
    let aspect_ratio = screen_width() / screen_height();

    let world_width = world.width as f32 * TILE_PIXEL;
    let world_height = world.height as f32 * TILE_PIXEL;

    world_height.min(world_width / aspect_ratio)
}

fn clamp_camera_zoom(visible_height: &mut f32, world: &World) {
    let maximum_visible_height = get_maximum_visible_height(world);

    let minimum_visible_height = MIN_CAMERA_VISIBLE_HEIGHT.min(maximum_visible_height);

    *visible_height = (*visible_height).clamp(minimum_visible_height, maximum_visible_height);
}

fn clamp_camera_position(camera_position: &mut Vec2, world: &World, visible_height: f32) {
    let aspect_ratio = screen_width() / screen_height();
    let visible_width = visible_height * aspect_ratio;

    let world_width = world.width as f32 * TILE_PIXEL;
    let world_height = world.height as f32 * TILE_PIXEL;

    let half_width = visible_width / 2.0;
    let half_height = visible_height / 2.0;

    camera_position.x = camera_position
        .x
        .clamp(half_width, world_width - half_width);

    camera_position.y = camera_position
        .y
        .clamp(half_height, world_height - half_height);
}

fn update_camera_zoom(visible_height: &mut f32, camera_position: &mut Vec2, world: &World) {
    let (_, wheel_y) = mouse_wheel();

    if wheel_y == 0.0 {
        return;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_screen = vec2(mouse_x, mouse_y);

    let camera_before = create_camera(*camera_position, *visible_height);

    let mouse_world_before = camera_before.screen_to_world(mouse_screen);

    *visible_height *= ZOOM_FACTOR.powf(wheel_y);

    clamp_camera_zoom(visible_height, world);

    let camera_after = create_camera(*camera_position, *visible_height);

    let mouse_world_after = camera_after.screen_to_world(mouse_screen);

    *camera_position += mouse_world_before - mouse_world_after;
}

fn update_keyboard_movement(camera_position: &mut Vec2, visible_height: f32) {
    let mut direction = Vec2::ZERO;

    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        direction.y -= 1.0;
    }

    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        direction.y += 1.0;
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

    let zoom_adjusted_speed = CAMERA_SPEED * (visible_height / DEFAULT_CAMERA_VISIBLE_HEIGHT);

    *camera_position += direction * zoom_adjusted_speed * get_frame_time();
}

fn update_mouse_drag(
    camera_position: &mut Vec2,
    visible_height: f32,
    previous_mouse_position: &mut Option<Vec2>,
) {
    let (mouse_x, mouse_y) = mouse_position();
    let current_mouse_position = vec2(mouse_x, mouse_y);

    if is_mouse_button_pressed(MouseButton::Middle) {
        *previous_mouse_position = Some(current_mouse_position);
    }

    if is_mouse_button_down(MouseButton::Middle) {
        if let Some(previous_position) = *previous_mouse_position {
            let mouse_delta = current_mouse_position - previous_position;

            let aspect_ratio = screen_width() / screen_height();

            let visible_width = visible_height * aspect_ratio;

            let world_units_per_screen_x = visible_width / screen_width();

            let world_units_per_screen_y = visible_height / screen_height();

            camera_position.x -= mouse_delta.x * world_units_per_screen_x;

            camera_position.y -= mouse_delta.y * world_units_per_screen_y;
        }

        *previous_mouse_position = Some(current_mouse_position);
    }

    if is_mouse_button_released(MouseButton::Middle) {
        *previous_mouse_position = None;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
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

    let mut camera_visible_height = DEFAULT_CAMERA_VISIBLE_HEIGHT;

    let mut previous_mouse_position: Option<Vec2> = None;

    loop {
        clamp_camera_zoom(&mut camera_visible_height, &world);

        update_camera_zoom(&mut camera_visible_height, &mut camera_position, &world);

        update_keyboard_movement(&mut camera_position, camera_visible_height);

        update_mouse_drag(
            &mut camera_position,
            camera_visible_height,
            &mut previous_mouse_position,
        );

        clamp_camera_position(&mut camera_position, &world, camera_visible_height);

        let camera = create_camera(camera_position, camera_visible_height);

        clear_background(BLACK);

        set_camera(&camera);
        draw_world(&world);

        set_default_camera();

        next_frame().await;
    }
}
