mod camera;
mod generation;
mod organism;
mod render;
mod resources;
mod ui;
mod world;

use crate::camera::{
    clamp_camera_position, create_camera, mouse_world_position, smooth_camera, update_camera_target,
};

use crate::generation::generate_world;

use crate::organism::initialize_organism;

use crate::render::{draw_organism, draw_world};

use crate::resources::Resources;

use crate::ui::draw_hud;

use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

const WORLD_WIDTH: usize = 80;
const WORLD_HEIGHT: usize = 50;

pub(crate) const TILE_PIXEL: f32 = 8.0;

pub(crate) const DEFAULT_CAMERA_VISIBLE_HEIGHT: f32 = 350.0;

const WORLD_SEED: &str = "Kevin'sIcyLife";

const SIMULATION_SEED: u64 = 42;

fn window_conf() -> Conf {
    Conf {
        window_title: "IcyMap".to_string(),

        window_width: WINDOW_WIDTH,

        window_height: WINDOW_HEIGHT,

        window_resizable: false,

        fullscreen: false,

        high_dpi: false,

        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    /*
     * Separate deterministic RNG seed
     * for organism simulation.
     */
    macroquad::rand::srand(SIMULATION_SEED);

    /*
     * WORLD
     */
    let world = generate_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_SEED);

    /*
     * RESOURCES
     */
    let mut resources = Resources::from_world(&world);

    /*
     * POPULATION
     *
     * Starts with exactly one organism.
     */
    let mut organisms = vec![initialize_organism(&world)];

    let mut next_organism_id: u64 = 2;

    /*
     * CAMERA
     */
    let world_center = vec2(
        world.width as f32 * TILE_PIXEL / 2.0,
        world.height as f32 * TILE_PIXEL / 2.0,
    );

    let mut camera_position = world_center;

    let mut target_camera_position = world_center;

    loop {
        let delta_time = get_frame_time().min(0.05);

        // ==============================
        // SIMULATION
        // ==============================

        /*
         * Nutrient regeneration.
         */
        resources.regenerate(delta_time);

        /*
         * Children cannot be pushed
         * directly into organisms while
         * we're iterating mutably over it.
         */
        let mut newborns = Vec::new();

        for organism in &mut organisms {
            /*
             * Milestone 7:
             *
             * Sample surrounding
             * nutrients and move with
             * weighted chemotaxis.
             */
            organism.move_with_chemotaxis(&world, &resources, delta_time);

            /*
             * Consume nutrients from
             * current tile.
             */
            organism.feed(&mut resources, delta_time);

            /*
             * Age + metabolism + death.
             */
            organism.life_living(delta_time);

            /*
             * Binary fission.
             */
            if organism.can_divide() {
                let child = organism.divide(next_organism_id, &world);

                next_organism_id += 1;

                newborns.push(child);
            }
        }

        /*
         * Add children only after
         * iteration finishes.
         */
        organisms.extend(newborns);

        /*
         * Remove dead organisms.
         */
        organisms.retain(|organism| organism.alive);

        // ==============================
        // CAMERA
        // ==============================

        update_camera_target(&mut target_camera_position, &world, delta_time);

        smooth_camera(&mut camera_position, target_camera_position, delta_time);

        clamp_camera_position(&mut camera_position, &world, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        let camera = create_camera(camera_position, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        // ==============================
        // TILE INSPECTOR
        // ==============================

        let mouse_world = mouse_world_position(&camera);

        let tile_x = (mouse_world.x / TILE_PIXEL).floor() as isize;

        let tile_y = (mouse_world.y / TILE_PIXEL).floor() as isize;

        let hovered_tile = if tile_x >= 0
            && tile_y >= 0
            && tile_x < world.width as isize
            && tile_y < world.height as isize
        {
            Some((
                tile_x as usize,
                tile_y as usize,
                world.get_world_tile(tile_x as usize, tile_y as usize),
            ))
        } else {
            None
        };

        // ==============================
        // RENDER
        // ==============================

        clear_background(BLACK);

        set_camera(&camera);

        draw_world(&world, camera_position, DEFAULT_CAMERA_VISIBLE_HEIGHT);

        for organism in &organisms {
            draw_organism(organism);
        }

        set_default_camera();

        draw_hud(hovered_tile);

        next_frame().await;
    }
}
