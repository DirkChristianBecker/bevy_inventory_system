mod systems {
    pub mod button_system;
    pub mod window_resized_system;
    pub mod show_hide_system;
    pub mod mouse_scroll_system;
    pub mod reset_icons_system;
}

mod inventory_components;
mod inventory_background_tiles;
mod inventory_plugin;
mod inventory_state;
mod inventory_style;
mod marker;
mod inventory_events;
mod states;

pub mod prelude {
    pub use crate::inventory_plugin::InventoryPlugin;

    // Components
    pub use crate::inventory_components::IconComponent;
    pub use crate::marker::UiCameraComponent;
    pub use crate::marker::UiRootComponent;
    pub use crate::inventory_components::ScrollPanel;

    // Events
    pub use crate::inventory_events::InventoryIconClicked;
    pub use crate::inventory_events::InventoryIconMouseEnter;
    pub use crate::inventory_events::InventoryIconMouseExit;
    pub use crate::inventory_events::ResetInventoryIcons;
    pub use crate::inventory_events::ToggleInventory;

    // Resources
    pub use crate::inventory_background_tiles::InventoryBackgroundTiles;
    pub use crate::inventory_state::InventoryState;
    pub use crate::inventory_style::InventoryStyle;

    // States
    pub use crate::states::InventoryStates;
}
