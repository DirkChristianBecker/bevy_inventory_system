use bevy::prelude::*;

/// Represents an event when an inventory icon is clicked.
#[derive(Event, Default)]
pub struct InventoryIconClicked {
    x: usize,
    y: usize,
}

impl InventoryIconClicked {
    /// Creates a new instance of `InventoryIconClicked` with the given `x` and `y` coordinates.
    ///
    /// # Arguments
    /// - `x`: The x coordinate of the icon.
    /// - `y`: The y coordinate of the icon.
    ///
    /// # Example
    /// ```
    /// use bevy_inventory_system::prelude::InventoryIconClicked;
    /// let icon_clicked = InventoryIconClicked::new(5, 10);
    /// assert_eq!(icon_clicked.x(), 5);
    /// assert_eq!(icon_clicked.y(), 10);
    /// ```
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

/// An event, when the mouse enters an inventory icon.
///
/// The event contains the x and y coordinates of the inventory icon.
///
/// # Fields
/// - `x`: The x coordinate of the icon.
/// - `y`: The y coordinate of the icon.
#[derive(Event, Default)]
pub struct InventoryIconMouseEnter {
    x: usize,
    y: usize,
}

impl InventoryIconMouseEnter {
    /// Creates a new instance of `InventoryIconMouseEnter` with the given `x` and `y` coordinates.
    ///
    /// # Arguments
    /// - `x`: The x coordinate of the icon.
    /// - `y`: The y coordinate of the icon.
    ///
    /// # Example
    /// ```
    /// use bevy_inventory_system::prelude::InventoryIconMouseEnter;
    /// let mouse_enter_event = InventoryIconMouseEnter::new(3, 7);
    /// assert_eq!(mouse_enter_event.x(), 3);
    /// assert_eq!(mouse_enter_event.y(), 7);
    /// ```
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

/// Represents an event when the mouse exits an inventory icon.
///
/// The event contains the x and y coordinates of the icon.
///
/// # Fields
/// - `x`: The x coordinate of the icon.
/// - `y`: The y coordinate of the icon.
#[derive(Event, Default)]
pub struct InventoryIconMouseExit {
    x: usize,
    y: usize,
}

impl InventoryIconMouseExit {
    /// Creates a new instance of `InventoryIconMouseExit` with the given `x` and `y` coordinates.
    ///
    /// # Arguments
    /// - `x`: The x coordinate of the icon.
    /// - `y`: The y coordinate of the icon.
    ///
    /// # Example
    /// ```
    /// use bevy_inventory_system::prelude::InventoryIconMouseExit;
    /// let mouse_exit_event = InventoryIconMouseExit::new(2, 8);
    /// assert_eq!(mouse_exit_event.x(), 2);
    /// assert_eq!(mouse_exit_event.y(), 8);
    /// ```
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

/// Represents an event to reset inventory icons.
#[derive(Event, Default)]
pub struct ResetInventoryIcons;

#[derive(Event, Default)]
pub struct ToggleInventory;


