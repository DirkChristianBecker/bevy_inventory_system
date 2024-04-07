use crate::components::IconComponent;
use crate::marker::UiRootComponent;
use crate::ui_events::{InventoryIconClicked, InventoryIconMouseEnter, InventoryIconMouseExit, ResetInventoryIcons};

use crate::prelude::*;

use bevy::prelude::*;
use bevy::window::WindowResized;

fn on_window_size_changed(
    mut q2: Query<&mut Style, With<UiRootComponent>>,
    inv_style : ResMut<InventoryStyle>,
    mut inv_state : ResMut<InventoryState>,
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

                    inv_state.left = ev.width * (layout_width / ev.width);
                    inv_state.top = ev.height * (layout_height / ev.height);
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

fn setup(
    mut commands: Commands,
    inv_style: Res<InventoryStyle>,
    backgrounds: Res<InventoryBackgroundTiles>,
    assets: Res<AssetServer>,
) {
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

    let root_node = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                top: Val::Percent(25.0),
                left: Val::Percent(25.0),
                ..default()
            },
            ..default()
        },
        UiRootComponent {},
    );

    let root_id = commands.spawn(root_node).id();
    let mut children: Vec<Entity> = Vec::new();

    for x in 0..inv_style.get_rows() {
        for y in 0..inv_style.get_columns() {
            let icon = backgrounds.get_random_tile();
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
                    ..default() },
                )
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

    commands.entity(root_id).push_children(&children);
}

fn icon_clicked_system(mut evt: EventReader<InventoryIconClicked>) {
    for _ in evt.read() {
        //info!("Item ({}, {}) clicked!", ev.x(), ev.y());
    }
}

fn icon_entered_system(mut evt: EventReader<InventoryIconMouseEnter>) {
    for _ in evt.read() {
        // info!("Item ({}, {}) entered!", ev.x(), ev.y());
    }
}

fn icon_exit_system(mut evt: EventReader<InventoryIconMouseExit>) {
    for _ in evt.read() {
        // info!("Item ({}, {}) exited!", ev.x(), ev.y());
    }
}

/// This is run after an element has been clicked, to ensure,
/// that the previously select item will be reset.
fn reset_icons_events(
    mut evt: EventReader<ResetInventoryIcons>,
    inv_state : ResMut<InventoryState>,
    inv_style : ResMut<InventoryStyle>,
    mut icons : Query<
    (
        &mut BackgroundColor,
        &mut BorderColor,
        &mut IconComponent,
    )>) {
    for _ in evt.read() {
        for (mut color, mut border, icon) in &mut icons {           
            let v = icon.get_grid_position();

            if let Some(s) = inv_state.selected_icon {
                if s.x == v.x && s.y == v.y {
                    *color = (*inv_style.get_selected_background_color()).into();
                    border.0 = *inv_style.get_selected_border_color();

                    continue;
                }
            }

            if let Some(s) = inv_state.hovered_icon {
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

#[allow(clippy::complexity)]
fn button_system( 
    inv_style : ResMut<InventoryStyle>,
    mut inv_state : ResMut<InventoryState>,
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
    mut reset : EventWriter<ResetInventoryIcons>,
) {
    for (interaction, mut color, mut border_color, icon) in &mut interaction_query {
        let v = icon.get_grid_position();

        match *interaction {
            Interaction::Pressed => {
                *color = inv_style.get_selected_background_color().clone().into();
                border_color.0 = inv_style.get_selected_border_color().clone();

                let event = InventoryIconClicked::new(v.x as usize, v.y as usize);
                clicks.send(event);

                inv_state.selected_icon = Some(v);
                reset.send(ResetInventoryIcons { });
            }

            Interaction::Hovered => {
                // Is this item selected? If so do not draw as
                // hovered.
                if let Some(e) = inv_state.selected_icon {
                    if e.x == v.x && e.y == v.y {
                        *color = inv_style.get_selected_background_color().clone().into();
                        border_color.0 = inv_style.get_selected_border_color().clone();
                    }
                    else 
                    {
                        *color = inv_style.get_hover_background_color().clone().into();
                        border_color.0 = inv_style.get_hover_border_color().clone();
                    }
                }                

                // Mouse exited
                if let Some(e) = inv_state.hovered_icon {
                    if e.x == v.x && e.y == v.y {
                        continue;
                    }

                    let evt = InventoryIconMouseExit::new(v.x as usize, v.y as usize);
                    unhovered.send(evt);
                }

                // Mouse entered
                let evt2 = InventoryIconMouseEnter::new(v.x as usize, v.y as usize);
                hovered.send(evt2);

                inv_state.hovered_icon = Some(v);
            }

            Interaction::None => {
                *color = inv_style.get_normal_background_color().clone().into();
                border_color.0 = inv_style.get_normal_border_color().clone();

                // Mous exited
                if let Some(e) = inv_state.hovered_icon {
                    if e.x == v.x && e.y == v.y {
                        let evt = InventoryIconMouseExit::new(v.x as usize, v.y as usize);
                        unhovered.send(evt);

                        inv_state.hovered_icon = None;
                    }
                }

                // Is this button selected?
                if let Some(e) = inv_state.selected_icon {
                    if e.x == v.x && e.y == v.y {
                        *color = inv_style.get_selected_background_color().clone().into();
                        border_color.0 = inv_style.get_selected_border_color().clone();
                    }
                }
            }
        }
    }
}

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Events
        app.add_event::<InventoryIconClicked>();
        app.add_event::<InventoryIconMouseEnter>();
        app.add_event::<InventoryIconMouseExit>();
        app.add_event::<ResetInventoryIcons>();

        // Resources
        app.init_resource::<InventoryStyle>();
        app.init_resource::<InventoryBackgroundTiles>();
        app.init_resource::<InventoryState>();
        
        // Systems
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            button_system, 
            on_window_size_changed, 
            icon_clicked_system,
            icon_entered_system,
            icon_exit_system,
            reset_icons_events,
        ));
    }
}
