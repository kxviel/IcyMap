use macroquad::prelude::*;

pub(crate) struct TerrainTextures {
    pub(crate) land: Texture2D,
    pub(crate) mountain: Texture2D,
    pub(crate) ocean: Texture2D,
    pub(crate) desert: Texture2D,
    pub(crate) trees: [Texture2D; 4],
}

impl TerrainTextures {
    pub(crate) async fn load() -> Self {
        let tree_1 = load_texture("assets/objects/tree-sprite-16x32-001.png")
            .await
            .expect("Failed to load tree sprite 001");

        let tree_2 = load_texture("assets/objects/tree-sprite-16x32-002.png")
            .await
            .expect("Failed to load tree sprite 002");

        let tree_3 = load_texture("assets/objects/tree-sprite-16x32-003.png")
            .await
            .expect("Failed to load tree sprite 003");

        let tree_4 = load_texture("assets/objects/tree-sprite-16x32-004.png")
            .await
            .expect("Failed to load tree sprite 004");

        let trees = [tree_1, tree_2, tree_3, tree_4];

        let land = load_texture("assets/terrain/land 32x32.png")
            .await
            .expect("Failed to load land texture");

        let mountain = load_texture("assets/terrain/mountain 32x32.png")
            .await
            .expect("Failed to load mountain texture");

        let ocean = load_texture("assets/terrain/ocean 32x32.png")
            .await
            .expect("Failed to load ocean texture");

        let desert = load_texture("assets/terrain/desert 32x32.png")
            .await
            .expect("Failed to load desert texture");

        for tree in &trees {
            tree.set_filter(FilterMode::Nearest);
        }

        land.set_filter(FilterMode::Linear);
        mountain.set_filter(FilterMode::Linear);
        ocean.set_filter(FilterMode::Linear);
        desert.set_filter(FilterMode::Linear);

        Self {
            land,
            mountain,
            ocean,
            desert,
            trees,
        }
    }
}
