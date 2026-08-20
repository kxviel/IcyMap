#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Biome {
    DeepWater,
    ShallowWater,
    Land,
    Mountain,
    Rot,
    Swamp,
    Volcano,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Terrain {
    Lava,
    Sand,
    Soil,
    Grassy,
    Rock,
    Snow,
    Mud,
    SwampWater,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Flora {
    Flower,
    Bush,
    DeadTree,
    MangroveTree,
    Reeds,
    ShortTree,
    TallTree,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Tile {
    pub(crate) height: f32,
    pub(crate) moisture: f32,
    pub(crate) biome: Biome,
    pub(crate) terrain: Option<Terrain>,
    pub(crate) flora: Option<Flora>,
}

pub(crate) struct World {
    pub(crate) width: usize,
    pub(crate) height: usize,
    tiles: Vec<Tile>,
}

impl World {
    pub(crate) fn from_tiles(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        assert_eq!(
            tiles.len(),
            width * height,
            "Tile count must match world dimensions",
        );

        Self {
            width,
            height,
            tiles,
        }
    }

    pub(crate) fn get_world_tile(&self, x: usize, y: usize) -> &Tile {
        let index = self.index(x, y);
        &self.tiles[index]
    }

    fn index(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);

        y * self.width + x
    }
}
