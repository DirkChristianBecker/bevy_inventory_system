use bevy::prelude::*;
use rand::Rng;

/// Contains the path to the image file on disk for every
/// tile we want to use. The inventory system assigns a random
/// tile to every inventory slot.
#[derive(Resource)]
pub struct InventoryBackgroundTiles {
    background_icons_small: Vec<String>,
}

impl Default for InventoryBackgroundTiles {
    fn default() -> Self {
        Self {
            background_icons_small: vec![
                "Apocalypse/HUD/Sprites/HUD/SPR_Background_Square_Tile_Grunge_Sml_01.png"
                    .to_string(),
                "Apocalypse/HUD/Sprites/HUD/SPR_Background_Square_Tile_Grunge_Sml_02.png"
                    .to_string(),
                "Apocalypse/HUD/Sprites/HUD/SPR_Background_Square_Tile_Grunge_Sml_LeftRight.png"
                    .to_string(),
            ],
        }
    }
}

impl InventoryBackgroundTiles {
    pub fn get_random_tile(&self) -> &String {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.background_icons_small.len());

        &self.background_icons_small[i]
    }
}