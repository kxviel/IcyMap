use crate::{
    TILE_PIXEL,
    world::{Flora, Terrain, Tile, World},
};
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct TileResource {
    pub(crate) nutrients: f32,
    pub(crate) capacity: f32,
    pub(crate) regeneration_rate: f32,
}

pub(crate) struct Resources {
    width: usize,
    height: usize,
    tiles: Vec<TileResource>,
}

impl Resources {
    pub(crate) fn from_world(world: &World) -> Self {
        let mut tiles = Vec::with_capacity(world.width * world.height);

        for y in 0..world.height {
            for x in 0..world.width {
                let tile = world.get_world_tile(x, y);

                tiles.push(create_tile_resource(tile));
            }
        }

        Self {
            width: world.width,
            height: world.height,
            tiles,
        }
    }

    pub(crate) fn regenerate(&mut self, delta_time: f32) {
        for tile in &mut self.tiles {
            tile.nutrients += tile.regeneration_rate * delta_time;

            tile.nutrients = tile.nutrients.min(tile.capacity);
        }
    }

    pub(crate) fn consume(&mut self, position: Vec2, amount: f32) -> f32 {
        let Some(index) = self.index_from_position(position) else {
            return 0.0;
        };

        let tile = &mut self.tiles[index];

        let consumed = amount.min(tile.nutrients);

        tile.nutrients -= consumed;

        consumed
    }

    pub(crate) fn nutrients_at_position(&self, position: Vec2) -> f32 {
        let Some(index) = self.index_from_position(position) else {
            return 0.0;
        };

        self.tiles[index].nutrients
    }

    fn index_from_position(&self, position: Vec2) -> Option<usize> {
        let x = (position.x / TILE_PIXEL).floor() as isize;

        let y = (position.y / TILE_PIXEL).floor() as isize;

        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return None;
        }

        Some(y as usize * self.width + x as usize)
    }
}

fn create_tile_resource(tile: &Tile) -> TileResource {
    let base_capacity = match tile.terrain {
        Some(Terrain::Grass) => 100.0,
        Some(Terrain::Soil) => 70.0,
        Some(Terrain::Sand) => 20.0,
        Some(Terrain::Rock) => 10.0,
        None => 0.0,
    };

    let moisture_factor = 0.5 + tile.moisture;

    /*
     * Flora is NOT food.
     *
     * For now it is only being used
     * as an indication that the local
     * environment may be richer.
     */
    let flora_bonus = match tile.flora {
        Some(Flora::Flower) => 5.0,
        Some(Flora::Bush) => 10.0,
        Some(Flora::ShortTree) => 15.0,
        Some(Flora::TallTree) => 20.0,
        None => 0.0,
    };

    let capacity = base_capacity * moisture_factor + flora_bonus;

    TileResource {
        nutrients: capacity,

        capacity,

        regeneration_rate: capacity * 0.01,
    }
}
