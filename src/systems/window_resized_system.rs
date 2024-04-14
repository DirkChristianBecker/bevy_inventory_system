use bevy::{prelude::*, window::WindowResized};
use crate::prelude::*;

/// This function `on_window_size_changed` is responsible for handling window resize events and updating the style of
/// UI elements accordingly.
///
/// It takes in several parameters:
/// - `q2`: A query that retrieves mutable references to the `Style` of UI elements with `UiRootComponent` marker component.
/// - `inv_style`: A mutable resource that holds the inventory style configuration.
/// - `inv_state`: A mutable resource that holds the current state of the inventory.
/// - `events`: An event reader that reads `WindowResized` events.
///
/// For each event the function iterates over the `Style` components and updates the
/// `top` and `left` properties of the `Style` component accordingly.
///
/// If the positioning value is `Val::Auto`, the function calculates the layout width and height based on the event
/// width and height and the total width and height. It then calculates the width and height percentages based on the layout
/// and assigns the result to `top` and `left` properties of the `Style` component.
///
/// If the positioning value is `Val::Percent`, the function sets the `top` and `left` properties of the `Style`
/// component to the given percentage value.
///
/// If the positioning value is `Val::Px`, the function sets the `top` and `left` properties of the `Style`
/// component to the given pixel value.
///
/// The function does nothing for other positioning values.
pub fn on_window_size_changed(
    mut q2: Query<&mut Style, With<UiRootComponent>>,
    inv_style: ResMut<InventoryStyle>,
    mut inv_state: ResMut<InventoryState>,
    mut events: EventReader<WindowResized>,
) {
    for ev in events.read() {
        for mut style in &mut q2 {
            match inv_style.get_positioning() {
                Val::Auto => {
                    let layout_width = (ev.width - inv_style.get_total_width() as f32) * 0.5;
                    let layout_height = (ev.height - inv_style.get_total_height() as f32) * 0.5;
                    let width_percent = (layout_width / ev.width) * 100.0;
                    let height_percent = (layout_height / ev.height) * 100.0;

                    style.top = Val::Percent(height_percent);
                    style.left = Val::Percent(width_percent);

                    inv_state.set_left(ev.width * (layout_width / ev.width));
                    inv_state.set_top(ev.height * (layout_height / ev.height));
                }

                Val::Percent(p) => {
                    style.top = Val::Percent(p);
                    style.left = Val::Percent(p);
                }

                Val::Px(p) => {
                    style.top = Val::Px(p);
                    style.left = Val::Px(p);
                }

                _ => {}
            }
        }
    }
}