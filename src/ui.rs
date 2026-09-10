use crate::noise::NoiseScales;
use crate::world::{Biome, Flora, Terrain, Tile};
use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui, hash, root_ui, widgets};
use std::ops::RangeInclusive;

pub const SIDEBAR_WIDTH: f32 = 270.0;
const SIDE_PADDING: f32 = 18.0;
const HEADER_HEIGHT: f32 = 50.0;

const SCALE_STEP: f32 = 0.001;
const SCALE_ROW_WIDTH: f32 = SIDEBAR_WIDTH - SIDE_PADDING * 2.0;
const SCALE_ROW_HEIGHT: f32 = 38.0;
const SCALE_ROW_GAP: f32 = 7.0;
const SCALE_BUTTON_X: f32 = 204.0;
const SCALE_BUTTON_WIDTH: f32 = 28.0;
const SCALE_BUTTON_HEIGHT: f32 = 15.0;

pub struct MapControls {
    pub seed: String,
    pub focused: bool,
    scales: [ScaleInput; 5],
    skin: Skin,
}

impl MapControls {
    pub fn new() -> Self {
        let defaults = NoiseScales::default();

        Self {
            seed: String::from("Kevin'sIcyMaps"),
            focused: false,
            scales: [
                ScaleInput::new("Height", defaults.height, 0.005..=0.150),
                ScaleInput::new("Moisture", defaults.moisture, 0.005..=0.150),
                ScaleInput::new("Flora density", defaults.flora_density, 0.005..=0.200),
                ScaleInput::new("Flora type", defaults.flora_type, 0.005..=0.250),
                ScaleInput::new("Terrain detail", defaults.terrain_detail, 0.005..=0.250),
            ],
            skin: controls_skin(),
        }
    }

    pub fn noise_scales(&self) -> NoiseScales {
        let [height, moisture, flora_density, flora_type, terrain_detail] = &self.scales;

        NoiseScales {
            height: height.value,
            moisture: moisture.value,
            flora_density: flora_density.value,
            flora_type: flora_type.value,
            terrain_detail: terrain_detail.value,
        }
    }

    pub fn update_focus(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.focused = mouse_is_over_sidebar();
        }
        if is_key_pressed(KeyCode::Tab) {
            self.focused = true;
        }
        if is_key_pressed(KeyCode::Escape) {
            self.focused = false;
            root_ui().clear_input_focus();
        }
    }
}

struct ScaleInput {
    label: &'static str,
    value: f32,
    input: String,
    range: RangeInclusive<f32>,
}

impl ScaleInput {
    fn new(label: &'static str, value: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            label,
            value,
            input: format!("{value:.3}"),
            range,
        }
    }

    fn normalize(&mut self) {
        let value = self
            .input
            .parse::<f32>()
            .ok()
            .filter(|value| value.is_finite())
            .unwrap_or(self.value);

        self.set_value(value);
    }

    fn set_value(&mut self, value: f32) {
        let clamped = value.clamp(*self.range.start(), *self.range.end());
        self.value = (clamped * 1000.0).round() / 1000.0;
        self.input = format!("{:.3}", self.value);
    }

    fn adjust(&mut self, delta: f32) {
        self.normalize();
        self.set_value(self.value + delta);
    }
}

fn controls_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_hex(0xe0e5e7))
        .build();

    let button_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_hex(0xeef2f3))
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .color(Color::from_hex(0x263136))
        .color_hovered(Color::from_hex(0x375257))
        .color_clicked(Color::from_hex(0x2a4145))
        .color_selected(Color::from_hex(0x367e86))
        .color_selected_hovered(Color::from_hex(0x3f919a))
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(Color::from_hex(0xeef2f3))
        .color(Color::from_hex(0x0d1215))
        .color_hovered(Color::from_hex(0x11181c))
        .color_clicked(Color::from_hex(0x11181c))
        .color_selected(Color::from_hex(0x365c61))
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

fn draw_scale_input(ui: &mut Ui, scale: &mut ScaleInput, position: Vec2) {
    let row_background = Color::from_hex(0x141b1f);
    let row_border = Color::from_hex(0x2b373d);
    let label_color = Color::from_hex(0xcdd4d7);

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
        scale.label,
        position.x + 10.0,
        position.y + 24.0,
        15.0,
        label_color,
    );

    let edited = widgets::Editbox::new(
        hash!("scale", scale.label),
        vec2(88.0, SCALE_ROW_HEIGHT - 8.0),
    )
    .position(position + vec2(112.0, 4.0))
    .margin(vec2(8.0, 7.0))
    .multiline(false)
    .filter(&is_scale_input_character)
    .ui(ui, &mut scale.input);

    let increase = widgets::Button::new("+")
        .position(position + vec2(SCALE_BUTTON_X, 3.0))
        .size(vec2(SCALE_BUTTON_WIDTH, SCALE_BUTTON_HEIGHT))
        .ui(ui);

    let decrease = widgets::Button::new("-")
        .position(position + vec2(SCALE_BUTTON_X, 20.0))
        .size(vec2(SCALE_BUTTON_WIDTH, SCALE_BUTTON_HEIGHT))
        .ui(ui);

    if edited
        && let Ok(parsed) = scale.input.parse::<f32>()
        && parsed.is_finite()
        && scale.range.contains(&parsed)
    {
        scale.value = parsed;
    }

    if increase {
        scale.adjust(SCALE_STEP);
    } else if decrease {
        scale.adjust(-SCALE_STEP);
    }
}

pub fn draw_controls(controls: &mut MapControls, hovered: Option<(usize, usize, &Tile)>) -> bool {
    draw_sidebar();
    if let Some((x, y, tile)) = hovered {
        draw_tile_inspector(x, y, tile);
    }

    let panel_x = screen_width() - SIDEBAR_WIDTH;
    let origin = vec2(panel_x + SIDE_PADDING, HEADER_HEIGHT + 18.0);
    let seed_position = origin + vec2(0.0, 56.0);
    let first_scale_position = origin + vec2(0.0, 132.0);
    let regenerate_position =
        first_scale_position + vec2(0.0, 5.0 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP) + 12.0);

    let section_color = Color::from_hex(0x89969d);
    let border = Color::from_hex(0x2b373d);

    draw_text(
        "WORLD GENERATION",
        origin.x,
        origin.y + 17.0,
        17.0,
        Color::from_hex(0xeef2f3),
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

    for (index, scale) in controls.scales.iter_mut().enumerate() {
        let position =
            first_scale_position + vec2(0.0, index as f32 * (SCALE_ROW_HEIGHT + SCALE_ROW_GAP));
        draw_scale_input(&mut ui, scale, position);
    }

    let regenerate = widgets::Button::new("REGENERATE MAP")
        .position(regenerate_position)
        .size(vec2(SCALE_ROW_WIDTH, 40.0))
        .selected(true)
        .ui(&mut ui)
        || (controls.focused && is_key_pressed(KeyCode::Enter));

    if regenerate {
        for scale in &mut controls.scales {
            scale.normalize();
        }
        controls.focused = false;
        ui.clear_input_focus();
    }

    ui.pop_skin();

    let help_y = regenerate_position.y + 62.0;
    for (index, line) in [
        "Enter to apply changes",
        "",
        "WASD / Arrows   Move",
        "Mouse wheel     Zoom",
        "Home            Reset view",
        "Esc             Leave input",
    ]
    .iter()
    .enumerate()
    {
        draw_text(
            line,
            origin.x,
            help_y + index as f32 * 18.0,
            14.0,
            section_color,
        );
    }

    regenerate
}

pub fn mouse_is_over_sidebar() -> bool {
    let (mouse_x, _) = mouse_position();
    mouse_x >= screen_width() - SIDEBAR_WIDTH
}

fn draw_sidebar() {
    let background = Color::new(0.04, 0.05, 0.06, 1.0);
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
    let panel_y = 12.0;

    let background = Color::new(0.04, 0.05, 0.06, 0.90);
    let border = Color::new(1.0, 1.0, 1.0, 0.12);
    let primary_text = Color::new(0.95, 0.96, 0.97, 1.0);
    let secondary_text = Color::new(0.72, 0.75, 0.78, 1.0);

    draw_rectangle(panel_x, panel_y, 220.0, 145.0, background);

    draw_rectangle_lines(panel_x, panel_y, 220.0, 145.0, 1.0, border);

    draw_text("TILE", panel_x + 12.0, panel_y + 22.0, 18.0, primary_text);

    let biome_text = match tile.biome {
        Biome::DeepWater => "Deep water",
        Biome::ShallowWater => "Shallow water",
        Biome::Land => "Land",
        Biome::Mountain => "Mountain",
    };
    let terrain_text = match tile.terrain {
        Some(Terrain::Sand) => "Sand",
        Some(Terrain::Soil) => "Soil",
        Some(Terrain::Grassy) => "Grass",
        Some(Terrain::Rock) => "Rock",
        Some(Terrain::Snow) => "Snow",
        None => "None",
    };

    let flora_text = match tile.flora {
        Some(Flora::Flower) => "Flower",
        Some(Flora::Bush) => "Bush",
        Some(Flora::ShortTree) => "Short tree",
        Some(Flora::TallTree) => "Tall tree",
        None => "None",
    };

    let lines = [
        format!("Position   {}, {}", x, y),
        format!("Height     {:.3}", tile.height),
        format!("Moisture   {:.3}", tile.moisture),
        format!("Biome      {}", biome_text),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_input_limits() {
        for (input, expected) in [
            ("", "0.021"),
            ("..", "0.021"),
            ("NaN", "0.021"),
            ("inf", "0.021"),
            ("0", "0.005"),
            ("9", "0.150"),
            ("0.0256", "0.026"),
        ] {
            let mut scale = ScaleInput::new("Height", 0.021, 0.005..=0.150);
            scale.input = input.to_string();
            scale.normalize();

            assert_eq!(scale.input, expected, "input: {input}");
            assert_eq!(scale.value, expected.parse::<f32>().unwrap());
        }
    }

    #[test]
    fn scale_button_steps() {
        let mut scale = ScaleInput::new("Height", 0.021, 0.005..=0.150);
        scale.input = "0.040".to_string();
        scale.adjust(SCALE_STEP);
        assert_eq!(scale.input, "0.041");

        for _ in 0..200 {
            scale.adjust(SCALE_STEP);
        }
        assert_eq!(scale.input, "0.150");

        for _ in 0..200 {
            scale.adjust(-SCALE_STEP);
        }
        assert_eq!(scale.input, "0.005");
    }
}
