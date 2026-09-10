use crate::{TILE_PIXEL, ui::SIDEBAR_WIDTH, world::World};
use macroquad::prelude::*;

const CAMERA_SPEED: f32 = 600.0;
const POSITION_SMOOTHING: f32 = 10.0;

fn map_viewport_width() -> f32 {
    (screen_width() - SIDEBAR_WIDTH).max(1.0)
}

fn map_viewport_aspect_ratio() -> f32 {
    map_viewport_width() / screen_height().max(1.0)
}

fn world_pixel_size(world: &World) -> Vec2 {
    vec2(
        world.width as f32 * TILE_PIXEL,
        world.height as f32 * TILE_PIXEL,
    )
}

pub fn create_camera(camera_position: Vec2, visible_height: f32) -> Camera2D {
    let visible_width = visible_height * map_viewport_aspect_ratio();

    let mut camera = Camera2D::from_display_rect(Rect::new(
        camera_position.x - visible_width / 2.0,
        camera_position.y - visible_height / 2.0,
        visible_width,
        visible_height,
    ));

    // Keep drawing and mouse coordinates on the same DPI scale.
    camera.zoom.x *= map_viewport_width() / screen_width().max(1.0);
    camera.offset.x = -SIDEBAR_WIDTH / screen_width().max(1.0);

    camera
}

pub fn clamp_camera_position(camera_position: &mut Vec2, world: &World, visible_height: f32) {
    let world_size = world_pixel_size(world);

    let visible_width = visible_height * map_viewport_aspect_ratio();

    camera_position.x = clamp_axis(camera_position.x, world_size.x, visible_width);
    camera_position.y = clamp_axis(camera_position.y, world_size.y, visible_height);
}

fn clamp_axis(position: f32, world_size: f32, visible_size: f32) -> f32 {
    let margin = (visible_size / 2.0).min(world_size / 2.0);
    position.clamp(margin, world_size - margin)
}

pub fn smooth_camera(camera_position: &mut Vec2, target_position: Vec2, delta_time: f32) {
    let position_factor = 1.0 - (-POSITION_SMOOTHING * delta_time).exp();

    *camera_position = (*camera_position).lerp(target_position, position_factor);
}

pub fn update_camera_target(target_position: &mut Vec2, delta_time: f32) {
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
        *target_position += direction * CAMERA_SPEED * delta_time;
    }
}
