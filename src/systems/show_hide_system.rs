use bevy::{a11y::{accesskit::{NodeBuilder, Role}, AccessibilityNode}, prelude::*};
use crate::prelude::*;

pub fn receive_toggle_inventory_events_shown(
    mut toggle_events: EventReader<ToggleInventory>,
    mut next_state: ResMut<NextState<InventoryStates>>,
) {
    for _ in toggle_events.read() {
        next_state.set(InventoryStates::Hidden);
    }
}

pub fn receive_toggle_inventory_events_hidden(
    mut toggle_events: EventReader<ToggleInventory>,
    mut next_state: ResMut<NextState<InventoryStates>>,
) {
    for _ in toggle_events.read() {
        next_state.set(InventoryStates::Shown);
    }
}

pub fn show(
    mut commands: Commands,
    inv_style: Res<InventoryStyle>,
    mut state : ResMut<InventoryState>,
    backgrounds: Res<InventoryBackgroundTiles>,
    assets: Res<AssetServer>,
) {
    if state.is_shown() { return; }
    state.set_is_shown(true);
    
    let height = inv_style.get_icon_size();
    let width = inv_style.get_icon_size();
    let margin = inv_style.get_icon_margin();
    let padding = inv_style.get_icon_padding();

    let slicer = TextureSlicer {
        border: BorderRect::square(15.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    // A root node for a UI layout.
    let root_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                top: Val::Percent(25.0),
                left: Val::Percent(25.0),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::clip(),
                ..default()
            },
            ..default()
        },
        UiRootComponent {},
        AccessibilityNode(NodeBuilder::new(Role::List)),
    );

    // Store the root id.
    let root_id = commands.spawn(root_node).id();

    let moving_panel = (
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Px(250.0),
                height: Val::Px(250.0),
                ..default()
            },
            ..default()
        },
        AccessibilityNode(NodeBuilder::new(Role::List)),
        ScrollPanel::default(),
    );
    let moving_panel_id = commands.spawn(moving_panel).id();
    commands.entity(root_id).add_child(moving_panel_id);

    let mut children: Vec<Entity> = Vec::new();
    for x in 0..inv_style.get_rows() {
        for y in 0..inv_style.get_columns() {
            let icon = backgrounds.get_random_tile_small();
            let image = assets.load(icon);

            let top = Val::Px((height * x as f32) + 2.0 * margin);
            let left = Val::Px((width * y as f32) + 2.0 * margin);

            let id = commands
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        top,
                        left,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(width - (margin + padding)),
                                    height: Val::Px(height - (margin + padding)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::FlexEnd,
                                    border: UiRect::all(Val::Px(inv_style.get_border_with())),
                                    ..default()
                                },
                                image: image.clone().into(),
                                ..default()
                            },
                            ImageScaleMode::Sliced(slicer.clone()),
                            IconComponent::new(x as i32, y as i32),
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("({}, {})", y, x),
                                TextStyle {
                                    font: assets.load(inv_style.get_normal_font().clone()),
                                    font_size: inv_style.get_normal_font_size(),
                                    color: *inv_style.get_normal_font_color(),
                                },
                            ));
                        });
                })
                .id();
            children.push(id);
        }
    }

    commands.entity(moving_panel_id).push_children(&children);
}

pub fn hide(
    mut commands: Commands,
    mut state : ResMut<InventoryState>,
    icons: Query<(&UiRootComponent, Entity)>,
) {
    state.set_is_shown(false);
    for (_, e) in &icons {
        commands.entity(e).despawn_recursive()
    }
}