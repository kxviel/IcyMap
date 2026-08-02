use macroquad::prelude::*;

const TOP_BAR_HEIGHT: f32 = 44.0;
const SIDE_PADDING: f32 = 18.0;

pub fn draw_hud(camera_visible_height: f32, default_visible_height: f32) {
    let background = Color::new(0.04, 0.05, 0.06, 0.88);
    let border = Color::new(1.0, 1.0, 1.0, 0.12);
    let primary_text = Color::new(0.95, 0.96, 0.97, 1.0);
    let secondary_text = Color::new(0.68, 0.72, 0.76, 1.0);

    draw_rectangle(0.0, 0.0, screen_width(), TOP_BAR_HEIGHT, background);

    draw_line(
        0.0,
        TOP_BAR_HEIGHT,
        screen_width(),
        TOP_BAR_HEIGHT,
        1.0,
        border,
    );

    draw_text("ICYMAP", SIDE_PADDING, 29.0, 23.0, primary_text);

    let zoom_percentage = default_visible_height / camera_visible_height * 100.0;

    let status = format!("ZOOM  {:.0}%     FPS  {}", zoom_percentage, get_fps(),);

    let status_size = measure_text(&status, None, 17, 1.0);

    draw_text(
        &status,
        screen_width() - status_size.width - SIDE_PADDING,
        28.0,
        17.0,
        secondary_text,
    );
}
