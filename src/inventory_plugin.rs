use crate::inventory_events::{
    InventoryIconClicked, InventoryIconMouseEnter, InventoryIconMouseExit, ResetInventoryIcons,
};

use crate::systems::*;
use crate::prelude::*;
use bevy::prelude::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<InventoryIconClicked>();
        app.add_event::<InventoryIconMouseEnter>();
        app.add_event::<InventoryIconMouseExit>();
        app.add_event::<ResetInventoryIcons>();
        app.add_event::<ToggleInventory>();

        // Resources
        app.init_resource::<InventoryStyle>();
        app.init_resource::<InventoryBackgroundTiles>();
        app.init_resource::<InventoryState>();

        // Systems
        app.add_systems(OnEnter(InventoryStates::Shown), show_hide_system::show);
        app.add_systems(OnEnter(InventoryStates::Hidden), show_hide_system::hide);

        app.add_systems(
            Update,
            (
                button_system::button_system.run_if(in_state(InventoryStates::Shown)),
                show_hide_system::receive_toggle_inventory_events_shown.run_if(in_state(InventoryStates::Shown)),
                show_hide_system::receive_toggle_inventory_events_hidden.run_if(in_state(InventoryStates::Hidden)),
                window_resized_system::on_window_size_changed.run_if(in_state(InventoryStates::Shown)),
                reset_icons_system::reset_icons_events.run_if(in_state(InventoryStates::Shown)),
                mouse_scroll_system::mouse_scroll.run_if(in_state(InventoryStates::Shown)),
            ),
        );

        // Initial state
        app.init_state::<InventoryStates>();
    }
}
