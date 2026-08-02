use crate::{DEFAULT_CAMERA_VISIBLE_HEIGHT, MIN_CAMERA_VISIBLE_HEIGHT, TILE_PIXEL, world::World};
use macroquad::prelude::*;

const CAMERA_SPEED: f32 = 600.0;

const POSITION_SMOOTHING: f32 = 10.0;
const ZOOM_IN_SMOOTHING: f32 = 16.0;
const ZOOM_OUT_SMOOTHING: f32 = 24.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ZoomLevel {
    FullWorld,
    Default,
    Close,
}

impl ZoomLevel {
    pub(crate) fn label(self) -> &'static str {
        match self {
            ZoomLevel::FullWorld => "WORLD",
            ZoomLevel::Default => "REGION",
            ZoomLevel::Close => "CLOSE",
        }
    }
}

fn screen_aspect_ratio() -> f32 {
    screen_width() / screen_height().max(1.0)
}

fn world_pixel_size(world: &World) -> Vec2 {
    vec2(
        world.width as f32 * TILE_PIXEL,
        world.height as f32 * TILE_PIXEL,
    )
}

fn zoom_height(level: ZoomLevel, world: &World) -> f32 {
    match level {
        ZoomLevel::FullWorld => {
            let world_size = world_pixel_size(world);
            let aspect_ratio = screen_aspect_ratio();

            world_size.y.max(world_size.x / aspect_ratio)
        }

        ZoomLevel::Default => DEFAULT_CAMERA_VISIBLE_HEIGHT,

        ZoomLevel::Close => MIN_CAMERA_VISIBLE_HEIGHT,
    }
}

pub(crate) fn create_camera(camera_position: Vec2, visible_height: f32) -> Camera2D {
    let visible_width = visible_height * screen_aspect_ratio();

    Camera2D::from_display_rect(Rect::new(
        camera_position.x - visible_width / 2.0,
        camera_position.y - visible_height / 2.0,
        visible_width,
        visible_height,
    ))
}

pub(crate) fn clamp_camera_position(
    camera_position: &mut Vec2,
    world: &World,
    visible_height: f32,
) {
    let world_size = world_pixel_size(world);

    let visible_width = visible_height * screen_aspect_ratio();

    if visible_width >= world_size.x {
        camera_position.x = world_size.x / 2.0;
    } else {
        let half_width = visible_width / 2.0;

        camera_position.x = camera_position
            .x
            .clamp(half_width, world_size.x - half_width);
    }

    if visible_height >= world_size.y {
        camera_position.y = world_size.y / 2.0;
    } else {
        let half_height = visible_height / 2.0;

        camera_position.y = camera_position
            .y
            .clamp(half_height, world_size.y - half_height);
    }
}

pub(crate) fn update_zoom_target(
    zoom_level: &mut ZoomLevel,
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

    *zoom_level = if wheel_y > 0.0 {
        match *zoom_level {
            ZoomLevel::FullWorld => ZoomLevel::Default,
            ZoomLevel::Default => ZoomLevel::Close,
            ZoomLevel::Close => ZoomLevel::Close,
        }
    } else {
        match *zoom_level {
            ZoomLevel::Close => ZoomLevel::Default,
            ZoomLevel::Default => ZoomLevel::FullWorld,
            ZoomLevel::FullWorld => ZoomLevel::FullWorld,
        }
    };

    *target_visible_height = zoom_height(*zoom_level, world);

    if *zoom_level == ZoomLevel::FullWorld {
        *target_position = world_pixel_size(world) / 2.0;
    } else {
        let camera_after = create_camera(*target_position, *target_visible_height);

        let mouse_world_after = camera_after.screen_to_world(mouse_screen);

        *target_position += mouse_world_before - mouse_world_after;
    }

    clamp_camera_position(target_position, world, *target_visible_height);
}

pub(crate) fn smooth_camera(
    camera_position: &mut Vec2,
    camera_visible_height: &mut f32,
    target_position: Vec2,
    target_visible_height: f32,
) {
    let delta_time = get_frame_time().min(0.05);

    let position_factor = 1.0 - (-POSITION_SMOOTHING * delta_time).exp();

    let zoom_smoothing = if target_visible_height > *camera_visible_height {
        ZOOM_OUT_SMOOTHING
    } else {
        ZOOM_IN_SMOOTHING
    };

    let zoom_factor = 1.0 - (-zoom_smoothing * delta_time).exp();

    *camera_position = (*camera_position).lerp(target_position, position_factor);

    *camera_visible_height += (target_visible_height - *camera_visible_height) * zoom_factor;
}

pub(crate) fn update_camera_target(
    target_position: &mut Vec2,
    target_visible_height: f32,
    world: &World,
) {
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

        let zoom_adjusted_speed =
            CAMERA_SPEED * target_visible_height / DEFAULT_CAMERA_VISIBLE_HEIGHT;

        *target_position += direction * zoom_adjusted_speed * get_frame_time();
    }

    clamp_camera_position(target_position, world, target_visible_height);
}
