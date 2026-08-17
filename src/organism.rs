use macroquad::math::{Vec2, vec2};

const METABOLISM_RATE: f32 = 5.0;

use crate::{
    TILE_PIXEL,
    world::{Biome, World},
};

#[derive(Clone, Debug)]
pub(crate) struct Organism {
    id: u32,
    pub(crate) position: Vec2,
    genome: Genome,
    age: f32,
    energy: f32,
    pub(crate) alive: bool,
}

#[derive(Clone, Debug)]
struct Gene {
    activity: f32,
}

#[derive(Clone, Debug)]
struct Genome {
    fts_z: Gene,
}

impl Organism {
    pub(crate) fn eat(&mut self) {
        self.energy += 20.0;
    }

    pub(crate) fn life_living(&mut self, delta_time: f32) {
        if !self.alive {
            return;
        }

        self.age += delta_time;
        self.energy -= METABOLISM_RATE * delta_time;

        if self.energy <= 0.0 {
            self.energy = 0.0;
            self.alive = false;
        }

        println!(
            "Age: {:.1}, Energy: {:.1}, Alive: {}",
            self.age, self.energy, self.alive
        );
    }
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
                    energy: 100.0,
                    alive: true,
                };
            }
        }
    }

    panic!("Could not find a valid tile for organism");
}
