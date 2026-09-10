#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    DeepWater,
    ShallowWater,
    Land,
    Mountain,
}

#[derive(Clone, Copy, Debug)]
pub enum Terrain {
    Sand,
    Soil,
    Grassy,
    Rock,
    Snow,
}

#[derive(Clone, Copy, Debug)]
pub enum Flora {
    Flower,
    Bush,
    ShortTree,
    TallTree,
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub height: f32,
    pub moisture: f32,
    pub biome: Biome,
    pub terrain: Option<Terrain>,
    pub flora: Option<Flora>,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl World {
    pub fn from_tiles(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
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

    pub fn tile(&self, x: usize, y: usize) -> &Tile {
        debug_assert!(x < self.width && y < self.height);
        &self.tiles[y * self.width + x]
    }
}
