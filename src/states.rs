use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InventoryStates {
    Shown,
    #[default]
    Hidden,
}