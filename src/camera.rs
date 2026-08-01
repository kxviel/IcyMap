use crate::{DEFAULT_CAMERA_VISIBLE_HEIGHT, MIN_CAMERA_VISIBLE_HEIGHT, TILE_PIXEL, world::World};
use macroquad::prelude::*;

const CAMERA_SPEED: f32 = 600.0;
const CAMERA_SMOOTHING: f32 = 10.0;

const ZOOM_FACTOR: f32 = 0.80;

pub fn create_camera(camera_position: Vec2, visible_height: f32) -> Camera2D {
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

pub fn clamp_camera_position(camera_position: &mut Vec2, world: &World, visible_height: f32) {
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

pub fn update_zoom_target(
    target_visible_height: &mut f32,
    target_position: &mut Vec2,
    world: &World,
) {
    let (_, wheel_y) = mouse_wheel();

    if wheel_y == 0.0 {
        return;
    }

    let mouse_screen: Vec2 = mouse_position().into();

    let camera_before = create_camera(*target_position, *target_visible_height);

    let mouse_world_before = camera_before.screen_to_world(mouse_screen);

    *target_visible_height *= ZOOM_FACTOR.powf(wheel_y);

    clamp_camera_zoom(target_visible_height, world);

    let camera_after = create_camera(*target_position, *target_visible_height);

    let mouse_world_after = camera_after.screen_to_world(mouse_screen);

    *target_position += mouse_world_before - mouse_world_after;

    clamp_camera_position(target_position, world, *target_visible_height);
}

pub fn smooth_camera(
    camera_position: &mut Vec2,
    camera_visible_height: &mut f32,
    target_position: Vec2,
    target_visible_height: f32,
) {
    let delta_time = get_frame_time().min(0.05);

    let smoothing = 1.0 - (-CAMERA_SMOOTHING * delta_time).exp();

    *camera_position = camera_position.lerp(target_position, smoothing);

    *camera_visible_height += (target_visible_height - *camera_visible_height) * smoothing;
}

pub fn update_camera_target(target_position: &mut Vec2, target_visible_height: f32, world: &World) {
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

        let zoom_adjusted_speed =
            CAMERA_SPEED * target_visible_height / DEFAULT_CAMERA_VISIBLE_HEIGHT;

        *target_position += direction * zoom_adjusted_speed * get_frame_time();
    }

    clamp_camera_position(target_position, world, target_visible_height);
}
