use crate::world::Tile;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};

const SIDEBAR_WIDTH: f32 = 200.0;
const SIDE_PADDING: f32 = 18.0;

const INSPECTOR_WIDTH: f32 = 210.0;
const INSPECTOR_HEIGHT: f32 = 145.0;

pub(crate) fn draw_hud(hovered_tile: Option<(usize, usize, &Tile)>) {
    draw_sidebar();

    if let Some((x, y, tile)) = hovered_tile {
        draw_tile_inspector(x, y, tile);
    }
}

fn draw_sidebar() {
    let background = Color::new(0.04, 0.05, 0.06, 0.88);
    let primary_text = Color::new(0.95, 0.96, 0.97, 1.0);
    let secondary_text = Color::new(0.68, 0.72, 0.76, 1.0);

    draw_rectangle(
        screen_width() - SIDEBAR_WIDTH,
        0.0,
        SIDEBAR_WIDTH,
        screen_height(),
        background,
    );

    draw_text(
        "ICYMAPS",
        screen_width() - SIDEBAR_WIDTH + SIDE_PADDING,
        29.0,
        23.0,
        primary_text,
    );

    let status = format!("FPS {}", get_fps());
    let status_size = measure_text(&status, None, 17, 1.0);

    draw_text(
        &status,
        screen_width() - status_size.width - SIDE_PADDING,
        28.0,
        17.0,
        secondary_text,
    );
}

fn draw_tile_inspector(x: usize, y: usize, tile: &Tile) {
    let panel_x = 12.0;
    let panel_y = 44.0 + 12.0;

    let background = Color::new(0.04, 0.05, 0.06, 0.90);
    let border = Color::new(1.0, 1.0, 1.0, 0.12);
    let primary_text = Color::new(0.95, 0.96, 0.97, 1.0);
    let secondary_text = Color::new(0.72, 0.75, 0.78, 1.0);

    draw_rectangle(
        panel_x,
        panel_y,
        INSPECTOR_WIDTH,
        INSPECTOR_HEIGHT,
        background,
    );

    draw_rectangle_lines(
        panel_x,
        panel_y,
        INSPECTOR_WIDTH,
        INSPECTOR_HEIGHT,
        1.0,
        border,
    );

    draw_text("TILE", panel_x + 12.0, panel_y + 22.0, 18.0, primary_text);

    let terrain_text = match tile.terrain {
        Some(terrain) => format!("{:?}", terrain),
        None => "None".to_string(),
    };

    let flora_text = match tile.flora {
        Some(flora) => format!("{:?}", flora),
        None => "None".to_string(),
    };

    let lines = [
        format!("Position   {}, {}", x, y),
        format!("Height     {:.3}", tile.height),
        format!("Moisture   {:.3}", tile.moisture),
        format!("Biome      {:?}", tile.biome),
        format!("Terrain    {}", terrain_text),
        format!("Flora      {}", flora_text),
    ];

    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            panel_x + 12.0,
            panel_y + 43.0 + index as f32 * 16.0,
            15.0,
            secondary_text,
        );
    }
}
