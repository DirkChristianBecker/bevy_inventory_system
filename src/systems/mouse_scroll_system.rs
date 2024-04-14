use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};
use crate::prelude::*;

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollPanel, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let mut shift_pressed = false;
        if keys.pressed(KeyCode::ShiftLeft) {
            shift_pressed = true;
        }
        if keys.pressed(KeyCode::ShiftRight) {
            shift_pressed = true;
        }

        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            if shift_pressed {
                let items_width = list_node.size().x;
                let container_width = query_node.get(parent.get()).unwrap().size().x;

                let max_scroll = (container_width - items_width).max(0.0);

                let dy = match mouse_wheel_event.unit {
                    MouseScrollUnit::Line => mouse_wheel_event.y * 20.0,
                    MouseScrollUnit::Pixel => mouse_wheel_event.y,
                };

                scrolling_list.y += dy;
                scrolling_list.y = scrolling_list.y.clamp(0.0, max_scroll);
                style.left = Val::Px(scrolling_list.y);
            } else {
                let items_height = list_node.size().y;
                let container_height = query_node.get(parent.get()).unwrap().size().y;

                let max_scroll = (items_height - container_height).max(0.);

                let dy = match mouse_wheel_event.unit {
                    MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                    MouseScrollUnit::Pixel => mouse_wheel_event.y,
                };

                scrolling_list.x += dy;
                scrolling_list.x = scrolling_list.x.clamp(-max_scroll, 0.0);
                style.top = Val::Px(scrolling_list.x);

                info!(
                    "Items height: {}, container height: {}",
                    items_height, container_height
                );
            }
        }
    }
}