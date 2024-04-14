use bevy::prelude::*;
use crate::prelude::*;

/// This is run after an element has been clicked, to ensure,
/// that the previously select item will be reset.
pub fn reset_icons_events(
    mut evt: EventReader<ResetInventoryIcons>,
    inv_state: ResMut<InventoryState>,
    inv_style: ResMut<InventoryStyle>,
    mut icons: Query<(&mut BackgroundColor, &mut BorderColor, &mut IconComponent)>,
) {
    for _ in evt.read() {
        for (mut color, mut border, icon) in &mut icons {
            let v = icon.get_grid_position();

            if let Some(s) = inv_state.get_selected_icon() {
                if s.x == v.x && s.y == v.y {
                    *color = (*inv_style.get_selected_background_color()).into();
                    border.0 = *inv_style.get_selected_border_color();

                    continue;
                }
            }

            if let Some(s) = inv_state.get_hovered_icon() {
                if s.x == v.x && s.y == v.y {
                    *color = (*inv_style.get_hover_background_color()).into();
                    border.0 = *inv_style.get_hover_border_color();

                    continue;
                }
            }

            *color = (*inv_style.get_normal_background_color()).into();
            border.0 = *inv_style.get_normal_border_color();
        }
    }
}