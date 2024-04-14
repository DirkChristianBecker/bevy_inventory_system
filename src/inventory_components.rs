use bevy::prelude::*;

/// Represents an IconComponent struct.
///
/// This struct is used to store information about an on-screen icon and its corresponding in-memory item.
/// It contains the `item_id` field, which is an `IVec2` representing the position of the item in a 2D array.
#[derive(Component, Debug)]
pub struct IconComponent {
    item_id: IVec2, // Items are managed by a 2d array. This maps an on screen icon to an in memory Item.
}

impl IconComponent {
    /// Creates a new instance of `IconComponent` with the specified `x` and `y` coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the item in the 2D array.
    /// * `y` - The y coordinate of the item in the 2D array.
    ///
    /// # Returns
    ///
    /// A new instance of `IconComponent` with the specified coordinates.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_inventory_system::prelude::IconComponent;
    /// let icon = IconComponent::new(1, 2);
    /// ```
    ///
    /// This will create a new `IconComponent` with the item located at position (1, 2) in the 2D array.
    pub fn new(x: i32, y: i32) -> Self {
        IconComponent {
            item_id: IVec2 { x, y },
        }
    }

    /// Returns the grid position of the `IconComponent`.
    ///
    /// This method retrieves the `item_id` field of the `IconComponent`, which represents the position of the item in a
    /// 2D array.
    ///
    /// # Returns
    /// The grid position of the `IconComponent` as an `IVec2`.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_inventory_system::prelude::IconComponent;
    /// let icon = IconComponent::new(1, 2);
    /// let grid_position = icon.get_grid_position();
    /// assert_eq!(grid_position.x, 1);
    /// assert_eq!(grid_position.y, 2);
    ///
    pub fn get_grid_position(&self) -> IVec2 {
        self.item_id
    }
}

/// Represents a ScrollPanel struct.
///
/// This struct is used to define the position of a scroll panel in a graphical user interface.
/// It contains the `x` and `y` coordinates of the scroll panel.
#[derive(Component, Debug)]
pub struct ScrollPanel {
    pub x: f32,
    pub y: f32,
}

/// Implements the `Default` trait for the `ScrollPanel` struct.
///
/// This implementation sets the default values for the `x` and `y` coordinates of a scroll panel.
/// The default values are both set to 0.0.
///
/// # Example
///
/// ```
/// use bevy_inventory_system::prelude::ScrollPanel;
/// let scroll_panel = ScrollPanel::default();
/// assert_eq!(scroll_panel.x, 0.0);
/// assert_eq!(scroll_panel.y, 0.0);
/// ```
///
/// This will create a new `ScrollPanel` instance with the default values for the `x` and `y` coordinates.

impl Default for ScrollPanel {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
