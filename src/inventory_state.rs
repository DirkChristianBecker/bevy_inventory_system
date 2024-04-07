use bevy::prelude::*;

#[derive(Resource)]
pub struct InventoryState {
    pub left : f32,
    pub top : f32,

    // Selection state
    pub selected_icon : Option<IVec2>,
    pub hovered_icon : Option<IVec2>,
}

impl Default for InventoryState {
    fn default() -> Self {
        Self {
            left : 0.0,
            top : 0.0,

            selected_icon : None,
            hovered_icon : None,
        }
    }
}
