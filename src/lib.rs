mod components;
mod inventory_plugin;
mod marker;
mod ui_events;
mod inventory_style;
mod inventory_state;
mod inventory_background_tiles;

pub mod prelude {
    pub use crate::inventory_plugin::InventoryPlugin;

    // Components
    pub use crate::marker::UiCameraComponent;
    pub use crate::marker::UiRootComponent;

    // Events
    pub use crate::ui_events::InventoryIconClicked;
    pub use crate::ui_events::InventoryIconMouseEnter;
    pub use crate::ui_events::InventoryIconMouseExit;
    pub use crate::ui_events::ResetInventoryIcons;

    // Resources
    pub use crate::inventory_style::InventoryStyle;
    pub use crate::inventory_state::InventoryState;
    pub use crate::inventory_background_tiles::InventoryBackgroundTiles;
}