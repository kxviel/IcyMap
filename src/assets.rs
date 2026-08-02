use macroquad::prelude::*;

pub(crate) struct TerrainTextures {
    pub(crate) lands: [Texture2D; 4],
    pub(crate) mountain: Texture2D,
    pub(crate) ocean: Texture2D,
    pub(crate) desert: Texture2D,
    pub(crate) trees: [Texture2D; 4],
}

async fn get_trees() -> [Texture2D; 4] {
    let tree_1 = load_texture("assets/objects/tree/tree-sprite-16x32-001.png")
        .await
        .expect("Failed to load tree sprite 001");

    let tree_2 = load_texture("assets/objects/tree/tree-sprite-16x32-002.png")
        .await
        .expect("Failed to load tree sprite 002");

    let tree_3 = load_texture("assets/objects/tree/tree-sprite-16x32-003.png")
        .await
        .expect("Failed to load tree sprite 003");

    let tree_4 = load_texture("assets/objects/tree/tree-sprite-16x32-004.png")
        .await
        .expect("Failed to load tree sprite 004");

    [tree_1, tree_2, tree_3, tree_4]
}

async fn get_land() -> [Texture2D; 4] {
    let land_1 = load_texture("assets/objects/land/land-16x16-001.png")
        .await
        .expect("Failed to load land sprite 001");

    let land_2 = load_texture("assets/objects/land/land-16x16-002.png")
        .await
        .expect("Failed to load land sprite 002");

    let land_3 = load_texture("assets/objects/land/land-16x16-003.png")
        .await
        .expect("Failed to load land sprite 003");

    let land_4 = load_texture("assets/objects/land/land-16x16-004.png")
        .await
        .expect("Failed to load land sprite 004");

    [land_1, land_2, land_3, land_4]
}

impl TerrainTextures {
    pub(crate) async fn load() -> Self {
        let trees = get_trees().await;
        let lands = get_land().await;

        for tree in &trees {
            tree.set_filter(FilterMode::Nearest);
        }

        for land in &lands {
            land.set_filter(FilterMode::Nearest);
        }

        let mountain = load_texture("assets/terrain/mountain 32x32.png")
            .await
            .expect("Failed to load mountain texture");

        let ocean = load_texture("assets/terrain/ocean 32x32.png")
            .await
            .expect("Failed to load ocean texture");

        let desert = load_texture("assets/terrain/desert 32x32.png")
            .await
            .expect("Failed to load desert texture");

        mountain.set_filter(FilterMode::Linear);
        ocean.set_filter(FilterMode::Linear);
        desert.set_filter(FilterMode::Linear);

        Self {
            lands,
            mountain,
            ocean,
            desert,
            trees,
        }
    }
}
