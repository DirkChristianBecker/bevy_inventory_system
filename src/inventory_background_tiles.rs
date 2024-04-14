use bevy::prelude::*;
use rand::Rng;

/// Represents a collection of tiles, that can be used as background images
/// in the inventory. The inventory system then assigns random
/// tiles to the inventory slots.
#[derive(Resource)]
pub struct InventoryBackgroundTiles {
    background_icons_small: Vec<String>,
}

impl Default for InventoryBackgroundTiles {
    /// Initializes the `InventoryBackgroundTiles` struct with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_inventory_system::prelude::InventoryBackgroundTiles;
    /// let test = InventoryBackgroundTiles::default();
    /// let random_tile = test.get_random_tile_small();
    /// println!("Random tile: {}", random_tile);
    /// ```
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
    /// This method returns a random small tile.
    pub fn get_random_tile_small(&self) -> &String {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.background_icons_small.len());

        &self.background_icons_small[i]
    }
}
