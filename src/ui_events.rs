use bevy::prelude::*;

#[derive(Event)]
pub struct InventoryIconClicked {
    x: usize,
    y: usize,
}

impl InventoryIconClicked {
    pub fn new(x: usize, y: usize) -> Self {
        InventoryIconClicked { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

#[derive(Event)]
pub struct InventoryIconMouseEnter {
    x: usize,
    y: usize,
}

impl InventoryIconMouseEnter {
    pub fn new(x: usize, y: usize) -> Self {
        InventoryIconMouseEnter { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

#[derive(Event)]
pub struct InventoryIconMouseExit {
    x: usize,
    y: usize,
}

impl InventoryIconMouseExit {
    pub fn new(x: usize, y: usize) -> Self {
        InventoryIconMouseExit { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
}

#[derive(Event)]
pub struct ResetInventoryIcons;