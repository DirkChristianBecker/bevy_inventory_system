use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct IconComponent {
    item_id : IVec2,        // Items are managed by a 2d array. This maps an on screen icon to an in memory Item.
}

impl IconComponent {
    pub fn new(x : i32, y : i32) -> Self {
        IconComponent {
            item_id : IVec2 {x, y },
        }
    }

    pub fn get_grid_position(&self) -> IVec2 { self.item_id }
}