use crate::{
    TILE_PIXEL,
    resources::Resources,
    world::{Biome, World},
};
use macroquad::prelude::*;
use macroquad::rand::gen_range;

const METABOLISM_RATE: f32 = 5.0;
const MAX_ENERGY: f32 = 100.0;

const MIN_DIVISION_AGE: f32 = 5.0;
const MIN_DIVISION_ENERGY: f32 = 60.0;

const NUTRIENT_INTAKE_RATE: f32 = 10.0;
const ENERGY_PER_NUTRIENT: f32 = 1.0;

const MOVEMENT_SPEED: f32 = 6.0;
const MOVEMENT_ENERGY_COST: f32 = 0.15;
const DIRECTION_CHANGE_INTERVAL: f32 = 0.75;

const SENSING_DISTANCE: f32 = TILE_PIXEL;
const CHEMOTAXIS_STRENGTH: f32 = 0.05;

#[derive(Clone, Debug)]
pub(crate) struct Gene {
    pub(crate) activity: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct Genome {
    pub(crate) fts_z: Gene,
}

#[derive(Clone, Debug)]
pub(crate) struct Organism {
    pub(crate) id: u64,
    pub(crate) position: Vec2,
    pub(crate) genome: Genome,
    pub(crate) age: f32,
    pub(crate) energy: f32,
    pub(crate) alive: bool,

    direction: Vec2,
    direction_timer: f32,
}

pub(crate) fn initialize_organism(world: &World) -> Organism {
    for y in 0..world.height {
        for x in 0..world.width {
            let tile = world.get_world_tile(x, y);

            if tile.biome == Biome::Land && tile.terrain.is_some() {
                let position = vec2(
                    x as f32 * TILE_PIXEL + TILE_PIXEL / 2.0,
                    y as f32 * TILE_PIXEL + TILE_PIXEL / 2.0,
                );

                return Organism {
                    id: 1,
                    position,

                    genome: Genome {
                        fts_z: Gene { activity: 0.5 },
                    },

                    age: 0.0,
                    energy: MAX_ENERGY,
                    alive: true,

                    direction: Vec2::ZERO,
                    direction_timer: 0.0,
                };
            }
        }
    }

    panic!("Could not find a valid tile for the first organism");
}

impl Organism {
    pub(crate) fn life_living(&mut self, delta_time: f32) {
        if !self.alive {
            return;
        }

        self.age += delta_time;

        self.lose_energy(METABOLISM_RATE * delta_time);
    }

    pub(crate) fn feed(&mut self, resources: &mut Resources, delta_time: f32) {
        if !self.alive {
            return;
        }

        let requested = NUTRIENT_INTAKE_RATE * delta_time;

        let consumed = resources.consume(self.position, requested);

        self.energy += consumed * ENERGY_PER_NUTRIENT;

        self.energy = self.energy.min(MAX_ENERGY);
    }

    pub(crate) fn can_divide(&self) -> bool {
        if !self.alive {
            return false;
        }

        let fts_z_activity = self.genome.fts_z.activity.clamp(0.0, 1.0);

        let division_age = MIN_DIVISION_AGE + (1.0 - fts_z_activity) * 5.0;

        self.age >= division_age && self.energy >= MIN_DIVISION_ENERGY
    }

    pub(crate) fn divide(&mut self, child_id: u64, world: &World) -> Organism {
        let child_energy = self.energy * 0.4;

        self.energy *= 0.6;
        self.age = 0.0;

        let child_position = find_child_position(self.position, child_id, world);

        Organism {
            id: child_id,

            position: child_position,

            genome: self.genome.clone(),

            age: 0.0,

            energy: child_energy,

            alive: true,

            direction: Vec2::ZERO,
            direction_timer: 0.0,
        }
    }

    pub(crate) fn move_with_chemotaxis(
        &mut self,
        world: &World,
        resources: &Resources,
        delta_time: f32,
    ) {
        if !self.alive {
            return;
        }

        self.direction_timer -= delta_time;

        if self.direction_timer <= 0.0 {
            self.direction = choose_weighted_direction(self.position, world, resources);

            self.direction_timer = DIRECTION_CHANGE_INTERVAL;
        }

        if self.direction.length_squared() == 0.0 {
            return;
        }

        let movement = self.direction * MOVEMENT_SPEED * delta_time;

        let candidate_position = self.position + movement;

        if is_valid_position(candidate_position, world) {
            self.position = candidate_position;

            self.lose_energy(movement.length() * MOVEMENT_ENERGY_COST);
        } else {
            self.direction_timer = 0.0;
        }
    }

    fn lose_energy(&mut self, amount: f32) {
        self.energy -= amount;

        if self.energy <= 0.0 {
            self.energy = 0.0;
            self.alive = false;

            println!("Organism {} died at age {:.2}", self.id, self.age);
        }
    }
}

fn choose_weighted_direction(position: Vec2, world: &World, resources: &Resources) -> Vec2 {
    let diagonal = 1.0 / 2.0_f32.sqrt();

    let directions = [
        vec2(1.0, 0.0),
        vec2(-1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(0.0, -1.0),
        vec2(diagonal, diagonal),
        vec2(diagonal, -diagonal),
        vec2(-diagonal, diagonal),
        vec2(-diagonal, -diagonal),
    ];

    let mut weights = [0.0_f32; 8];

    let mut total_weight = 0.0;

    for (index, direction) in directions.iter().enumerate() {
        let sample_position = position + *direction * SENSING_DISTANCE;

        if !is_valid_position(sample_position, world) {
            continue;
        }

        let nutrients = resources.nutrients_at_position(sample_position);

        /*
         * Base weight 1.0 means even a poor
         * direction can still be selected.
         *
         * More nutrients = higher probability.
         */
        let weight = 1.0 + nutrients * CHEMOTAXIS_STRENGTH;

        weights[index] = weight;
        total_weight += weight;
    }

    if total_weight <= 0.0 {
        return Vec2::ZERO;
    }

    let mut choice = gen_range(0.0, total_weight);

    for (index, weight) in weights.iter().enumerate() {
        if *weight <= 0.0 {
            continue;
        }

        if choice <= *weight {
            return directions[index];
        }

        choice -= *weight;
    }

    directions[0]
}

fn find_child_position(parent_position: Vec2, child_id: u64, world: &World) -> Vec2 {
    let distance = TILE_PIXEL * 0.25;

    let offsets = match child_id % 4 {
        0 => [
            vec2(distance, 0.0),
            vec2(-distance, 0.0),
            vec2(0.0, distance),
            vec2(0.0, -distance),
        ],

        1 => [
            vec2(-distance, 0.0),
            vec2(distance, 0.0),
            vec2(0.0, -distance),
            vec2(0.0, distance),
        ],

        2 => [
            vec2(0.0, distance),
            vec2(0.0, -distance),
            vec2(distance, 0.0),
            vec2(-distance, 0.0),
        ],

        _ => [
            vec2(0.0, -distance),
            vec2(0.0, distance),
            vec2(-distance, 0.0),
            vec2(distance, 0.0),
        ],
    };

    for offset in offsets {
        let candidate = parent_position + offset;

        if is_valid_position(candidate, world) {
            return candidate;
        }
    }

    parent_position
}

fn is_valid_position(position: Vec2, world: &World) -> bool {
    let tile_x = (position.x / TILE_PIXEL).floor() as isize;

    let tile_y = (position.y / TILE_PIXEL).floor() as isize;

    if tile_x < 0 || tile_y < 0 || tile_x >= world.width as isize || tile_y >= world.height as isize
    {
        return false;
    }

    let tile = world.get_world_tile(tile_x as usize, tile_y as usize);

    tile.biome == Biome::Land && tile.terrain.is_some()
}
