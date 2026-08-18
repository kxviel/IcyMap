use crate::world::Tile;
use macroquad::prelude::*;
use macroquad::ui::{Id, Skin, Ui, hash, root_ui, widgets};

pub(crate) const SIDEBAR_WIDTH: f32 = 270.0;
const SIDE_PADDING: f32 = 18.0;
const HEADER_HEIGHT: f32 = 50.0;

const SCALE_STEP: f32 = 0.001;
const SCALE_ROW_WIDTH: f32 = SIDEBAR_WIDTH - SIDE_PADDING * 2.0;
const SCALE_ROW_HEIGHT: f32 = 38.0;
const SCALE_ROW_GAP: f32 = 7.0;
const SCALE_INPUT_X: f32 = 112.0;
const SCALE_INPUT_WIDTH: f32 = 88.0;
const SCALE_BUTTON_X: f32 = 204.0;
const SCALE_BUTTON_WIDTH: f32 = 28.0;
const SCALE_BUTTON_HEIGHT: f32 = 15.0;

const INSPECTOR_WIDTH: f32 = 210.0;
const INSPECTOR_HEIGHT: f32 = 145.0;

pub(crate) struct MapControls {
    pub(crate) seed: String,
    pub(crate) height_scale: f32,
    pub(crate) moisture_scale: f32,
    pub(crate) flora_density_scale: f32,
    pub(crate) flora_type_scale: f32,
    pub(crate) terrain_detail_scale: f32,
    height_scale_input: String,
    moisture_scale_input: String,
    flora_density_scale_input: String,
    flora_type_scale_input: String,
    terrain_detail_scale_input: String,
    skin: Skin,
}

impl MapControls {
    pub(crate) fn new() -> Self {
        Self {
            seed: String::from("Kevin'sIcyMaps"),
            height_scale: 0.021,
            moisture_scale: 0.014,
            flora_density_scale: 0.063,
            flora_type_scale: 0.140,
            terrain_detail_scale: 0.140,
            height_scale_input: String::from("0.021"),
            moisture_scale_input: String::from("0.014"),
            flora_density_scale_input: String::from("0.063"),
            flora_type_scale_input: String::from("0.140"),
            terrain_detail_scale_input: String::from("0.140"),
            skin: controls_skin(),
        }
    }

    fn normalize_scale_inputs(&mut self) {
        normalize_scale_input(
            &mut self.height_scale,
            &mut self.height_scale_input,
            0.005,
            0.150,
        );
        normalize_scale_input(
            &mut self.moisture_scale,
            &mut self.moisture_scale_input,
            0.005,
            0.150,
        );
        normalize_scale_input(
            &mut self.flora_density_scale,
            &mut self.flora_density_scale_input,
            0.005,
            0.200,
        );
        normalize_scale_input(
            &mut self.flora_type_scale,
            &mut self.flora_type_scale_input,
            0.005,
            0.250,
        );
        normalize_scale_input(
            &mut self.terrain_detail_scale,
            &mut self.terrain_detail_scale_input,
            0.005,
            0.250,
        );
    }
}

fn controls_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_rgba(224, 229, 231, 255))
        .build();

    let button_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_rgba(238, 242, 243, 255))
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .color(Color::from_rgba(38, 49, 54, 255))
        .color_hovered(Color::from_rgba(55, 82, 87, 255))
        .color_clicked(Color::from_rgba(42, 65, 69, 255))
        .color_selected(Color::from_rgba(54, 126, 134, 255))
        .color_selected_hovered(Color::from_rgba(63, 145, 154, 255))
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_rgba(238, 242, 243, 255))
        .color(Color::from_rgba(13, 18, 21, 255))
        .color_hovered(Color::from_rgba(17, 24, 28, 255))
        .color_clicked(Color::from_rgba(17, 24, 28, 255))
        .color_selected(Color::from_rgba(54, 92, 97, 255))
        .build();

    Skin {
        label_style,
        button_style,
        editbox_style,
        margin: 0.0,
        ..root_ui().default_skin()
    }
}

fn is_scale_input_character(character: char) -> bool {
    character.is_ascii_digit() || character == '.'
}

fn update_scale_from_input(value: &mut f32, input: &str, min: f32, max: f32) {
    if let Ok(parsed) = input.parse::<f32>()
        && parsed.is_finite()
        && (min..=max).contains(&parsed)
    {
        *value = parsed;
    }
}

fn normalize_scale_input(value: &mut f32, input: &mut String, min: f32, max: f32) {
    let parsed = input
        .parse::<f32>()
        .ok()
        .filter(|parsed| parsed.is_finite())
        .unwrap_or(*value)
        .clamp(min, max);

    *input = format!("{parsed:.3}");
    *value = input.parse().expect("formatted scale should be valid");
}

fn adjust_scale(value: &mut f32, input: &mut String, min: f32, max: f32, delta: f32) {
    normalize_scale_input(value, input, min, max);

    let adjusted = (*value + delta).clamp(min, max);
    *input = format!("{adjusted:.3}");
    *value = input.parse().expect("formatted scale should be valid");
}

fn draw_scale_input(
    ui: &mut Ui,
    id: Id,
    label: &str,
    position: Vec2,
    value: &mut f32,
    input: &mut String,
    range: std::ops::RangeInclusive<f32>,
) {
    let min = *range.start();
    let max = *range.end();

    let row_background = Color::from_rgba(20, 27, 31, 255);
    let row_border = Color::from_rgba(43, 55, 61, 255);
    let label_color = Color::from_rgba(205, 212, 215, 255);

    draw_rectangle(
        position.x,
        position.y,
        SCALE_ROW_WIDTH,
        SCALE_ROW_HEIGHT,
        row_background,
    );
    draw_rectangle_lines(
        position.x,
        position.y,
        SCALE_ROW_WIDTH,
        SCALE_ROW_HEIGHT,
        1.0,
        row_border,
    );
    draw_text(
        label,
        position.x + 10.0,
        position.y + 24.0,
        15.0,
        label_color,
    );

    let edited = widgets::Editbox::new(
        hash!(id, "input"),
        vec2(SCALE_INPUT_WIDTH, SCALE_ROW_HEIGHT - 8.0),
    )
    .position(position + vec2(SCALE_INPUT_X, 4.0))
    .margin(vec2(8.0, 7.0))
    .multiline(false)
    .filter(&is_scale_input_character)
    .ui(ui, input);

    let increase = widgets::Button::new("+")
        .position(position + vec2(SCALE_BUTTON_X, 3.0))
        .size(vec2(SCALE_BUTTON_WIDTH, SCALE_BUTTON_HEIGHT))
        .ui(ui);

    let decrease = widgets::Button::new("-")
        .position(position + vec2(SCALE_BUTTON_X, 20.0))
        .size(vec2(SCALE_BUTTON_WIDTH, SCALE_BUTTON_HEIGHT))
        .ui(ui);

    if edited {
        update_scale_from_input(value, input, min, max);
    }

    if increase {
        adjust_scale(value, input, min, max, SCALE_STEP);
    } else if decrease {
        adjust_scale(value, input, min, max, -SCALE_STEP);
    }
}

pub(crate) fn draw_controls(controls: &mut MapControls) -> bool {
    let panel_x = screen_width() - SIDEBAR_WIDTH;
    let origin = vec2(panel_x + SIDE_PADDING, HEADER_HEIGHT + 18.0);
    let seed_position = origin + vec2(0.0, 56.0);
    let first_scale_position = origin + vec2(0.0, 132.0);
    let regenerate_position =
        first_scale_position + vec2(0.0, 5.0 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP) + 12.0);

    let mut regenerate = false;

    let section_color = Color::from_rgba(137, 150, 157, 255);
    let border = Color::from_rgba(43, 55, 61, 255);

    draw_text(
        "WORLD GENERATION",
        origin.x,
        origin.y + 17.0,
        17.0,
        Color::from_rgba(238, 242, 243, 255),
    );
    draw_line(
        origin.x,
        origin.y + 30.0,
        origin.x + SCALE_ROW_WIDTH,
        origin.y + 30.0,
        1.0,
        border,
    );
    draw_text("SEED", origin.x, origin.y + 49.0, 13.0, section_color);
    draw_text(
        "NOISE SCALES",
        origin.x,
        origin.y + 119.0,
        13.0,
        section_color,
    );

    let mut ui = root_ui();
    ui.push_skin(&controls.skin);

    widgets::Editbox::new(hash!("seed"), vec2(SCALE_ROW_WIDTH, 34.0))
        .position(seed_position)
        .margin(vec2(10.0, 8.0))
        .multiline(false)
        .ui(&mut ui, &mut controls.seed);

    draw_scale_input(
        &mut ui,
        hash!("height_scale"),
        "Height",
        first_scale_position,
        &mut controls.height_scale,
        &mut controls.height_scale_input,
        0.005..=0.150,
    );

    draw_scale_input(
        &mut ui,
        hash!("moisture_scale"),
        "Moisture",
        first_scale_position + vec2(0.0, SCALE_ROW_HEIGHT + SCALE_ROW_GAP),
        &mut controls.moisture_scale,
        &mut controls.moisture_scale_input,
        0.005..=0.150,
    );

    draw_scale_input(
        &mut ui,
        hash!("flora_density_scale"),
        "Flora density",
        first_scale_position + vec2(0.0, 2.0 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP)),
        &mut controls.flora_density_scale,
        &mut controls.flora_density_scale_input,
        0.005..=0.200,
    );

    draw_scale_input(
        &mut ui,
        hash!("flora_type_scale"),
        "Flora type",
        first_scale_position + vec2(0.0, 3.0 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP)),
        &mut controls.flora_type_scale,
        &mut controls.flora_type_scale_input,
        0.005..=0.250,
    );

    draw_scale_input(
        &mut ui,
        hash!("terrain_detail_scale"),
        "Terrain detail",
        first_scale_position + vec2(0.0, 4.0 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP)),
        &mut controls.terrain_detail_scale,
        &mut controls.terrain_detail_scale_input,
        0.005..=0.250,
    );

    if widgets::Button::new("REGENERATE MAP")
        .position(regenerate_position)
        .size(vec2(SCALE_ROW_WIDTH, 40.0))
        .selected(true)
        .ui(&mut ui)
    {
        controls.normalize_scale_inputs();
        regenerate = true;
    }

    ui.pop_skin();

    regenerate
}

pub(crate) fn mouse_is_over_sidebar() -> bool {
    let (mouse_x, _) = mouse_position();
    mouse_x >= screen_width() - SIDEBAR_WIDTH
}

pub(crate) fn draw_hud(hovered_tile: Option<(usize, usize, &Tile)>) {
    draw_sidebar();

    if let Some((x, y, tile)) = hovered_tile {
        draw_tile_inspector(x, y, tile);
    }
}

fn draw_sidebar() {
    let background = Color::new(0.04, 0.05, 0.06, 0.96);
    let primary_text = Color::new(0.95, 0.96, 0.97, 1.0);
    let secondary_text = Color::new(0.68, 0.72, 0.76, 1.0);
    let border = Color::new(1.0, 1.0, 1.0, 0.10);
    let panel_x = screen_width() - SIDEBAR_WIDTH;

    draw_rectangle(panel_x, 0.0, SIDEBAR_WIDTH, screen_height(), background);
    draw_line(panel_x, 0.0, panel_x, screen_height(), 1.0, border);

    draw_text("ICYMAPS", panel_x + SIDE_PADDING, 31.0, 23.0, primary_text);

    let status = format!("FPS {}", get_fps());
    let status_size = measure_text(&status, None, 17, 1.0);

    draw_text(
        &status,
        screen_width() - status_size.width - SIDE_PADDING,
        30.0,
        17.0,
        secondary_text,
    );

    draw_line(
        panel_x + SIDE_PADDING,
        HEADER_HEIGHT,
        screen_width() - SIDE_PADDING,
        HEADER_HEIGHT,
        1.0,
        border,
    );
}

fn draw_tile_inspector(x: usize, y: usize, tile: &Tile) {
    let panel_x = 12.0;
    let panel_y = 56.0;

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
