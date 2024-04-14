use bevy::prelude::*;

use crate::{
    inventory_components::IconComponent,
    inventory_state::InventoryState,
    inventory_style::InventoryStyle,
    inventory_events::{
        InventoryIconClicked, InventoryIconMouseEnter, InventoryIconMouseExit, ResetInventoryIcons,
    },
};

/// This is the `button_system` function.
///
/// It function handles button interactions in an inventory UI.
///
/// The function iterates over the entities in the icons and handles different interactions based on the `Interaction`
/// component. 
/// 
/// If the interaction is `Pressed`, the function updates the color and border color of the button, 
/// sends an `InventoryIconClicked` event, sets the selected icon in the `inv_state`, and sends a `ResetInventoryIcons` event.
/// 
/// If the interaction is `Hovered`, the function checks if the item is selected. If the item is selected it does nothin. If not
/// it and updates the color and border color to the configured values in 'inv_style'. It also sends `InventoryIconMouseEnter` 
/// and `InventoryIconMouseExit` events based on the current and previously hovered icons.
/// 
/// If the interaction is `None`,  the function updates the color and border color of the button and sends a 
/// `InventoryIconMouseExit` event if the icon was previously hovered. It also checks if the button is selected 
/// and updates the color and border color accordingly.
///
#[allow(clippy::complexity)]
pub fn button_system(
    inv_style: ResMut<InventoryStyle>,
    mut inv_state: ResMut<InventoryState>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &IconComponent,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut clicks: EventWriter<InventoryIconClicked>,
    mut hovered: EventWriter<InventoryIconMouseEnter>,
    mut unhovered: EventWriter<InventoryIconMouseExit>,
    mut reset: EventWriter<ResetInventoryIcons>,
) {
    for (interaction, mut color, mut border_color, icon) in &mut interaction_query {
        let v = icon.get_grid_position();

        match *interaction {
            Interaction::Pressed => {
                *color = inv_style.get_selected_background_color().clone().into();
                border_color.0 = inv_style.get_selected_border_color().clone();

                let event = InventoryIconClicked::new(v.x as usize, v.y as usize);
                clicks.send(event);

                inv_state.set_selected_icon(Some(v));
                reset.send(ResetInventoryIcons {});
            }

            Interaction::Hovered => {
                // Is this item selected? If so do not draw as
                // hovered.
                if let Some(e) = inv_state.get_selected_icon() {
                    if e.x == v.x && e.y == v.y {
                        *color = inv_style.get_selected_background_color().clone().into();
                        border_color.0 = inv_style.get_selected_border_color().clone();
                    } else {
                        *color = inv_style.get_hover_background_color().clone().into();
                        border_color.0 = inv_style.get_hover_border_color().clone();
                    }
                }

                // Mouse exited
                if let Some(e) = inv_state.get_hovered_icon() {
                    if e.x == v.x && e.y == v.y {
                        continue;
                    }

                    let evt = InventoryIconMouseExit::new(v.x as usize, v.y as usize);
                    unhovered.send(evt);
                }

                // Mouse entered
                let evt2 = InventoryIconMouseEnter::new(v.x as usize, v.y as usize);
                hovered.send(evt2);

                inv_state.set_hovered_icon(Some(v));
            }

            Interaction::None => {
                *color = inv_style.get_normal_background_color().clone().into();
                border_color.0 = inv_style.get_normal_border_color().clone();

                // Mous exited
                if let Some(e) = inv_state.get_hovered_icon() {
                    if e.x == v.x && e.y == v.y {
                        let evt = InventoryIconMouseExit::new(v.x as usize, v.y as usize);
                        unhovered.send(evt);

                        inv_state.set_hovered_icon(None);
                    }
                }

                // Is this button selected?
                if let Some(e) = inv_state.get_selected_icon() {
                    if e.x == v.x && e.y == v.y {
                        *color = inv_style.get_selected_background_color().clone().into();
                        border_color.0 = inv_style.get_selected_border_color().clone();
                    }
                }
            }
        }
    }
}
