use bevy::prelude::*;

/// Represents the state of an inventory.
///
/// This struct contains the following fields:
/// - `left`: The left position of the inventory.
/// - `top`: The top position of the inventory.
/// - `selected_icon`: The currently selected icon in the inventory, if any.
/// - `hovered_icon`: The icon currently being hovered over in the inventory, if any.
#[derive(Resource)]
pub struct InventoryState {
    left: f32,
    top: f32,
    selected_icon: Option<IVec2>,
    hovered_icon: Option<IVec2>,

    is_shown : bool,
}

impl Default for InventoryState {
    /// Default implementation for InventoryState.
    ///
    /// The `left` and `top` fields are set to 0.0, while the `selected_icon` and `hovered_icon` fields are set to `None`.
    ///
    /// # Examples
    /// ```
    /// use bevy_inventory_system::prelude::InventoryState;
    /// use bevy::prelude::*;
    ///
    /// let mut inventory_state = InventoryState::default();
    /// assert_eq!(inventory_state.get_left(), 0.0);
    /// assert_eq!(inventory_state.get_top(), 0.0);
    /// assert!(inventory_state.get_selected_icon().is_none());
    /// assert!(inventory_state.get_hovered_icon().is_none());
    ///
    /// inventory_state.set_left(5.0);
    /// inventory_state.set_top(4.0);
    /// assert_eq!(inventory_state.get_left(), 5.0);
    /// assert_eq!(inventory_state.get_top(), 4.0);
    ///
    /// let v = IVec2 { x : 1, y : 2 };
    /// inventory_state.set_selected_icon(Some(v));
    /// inventory_state.set_hovered_icon(Some(v));
    /// assert_eq!(inventory_state.get_selected_icon().unwrap(), v);
    /// assert_eq!(inventory_state.get_hovered_icon().unwrap(), v);
    ///
    /// inventory_state.set_is_shown(true);
    /// assert!(inventory_state.is_shown());
    /// inventory_state.set_is_shown(false);
    /// assert!(!inventory_state.is_shown());
    /// ```
    fn default() -> Self {
        Self {
            left: 0.0,
            top: 0.0,

            selected_icon: None,
            hovered_icon: None,

            is_shown : false,
        }
    }
}

impl InventoryState {
    // Getter for `left`
    pub fn get_left(&self) -> f32 {
        self.left
    }

    // Setter for `left`
    pub fn set_left(&mut self, value: f32) {
        self.left = value;
    }

    // Getter for `top`
    pub fn get_top(&self) -> f32 {
        self.top
    }

    // Setter for `top`
    pub fn set_top(&mut self, value: f32) {
        self.top = value;
    }

    // Getter for `selected_icon`
    pub fn get_selected_icon(&self) -> Option<IVec2> {
        self.selected_icon
    }

    // Setter for `selected_icon`
    pub fn set_selected_icon(&mut self, value: Option<IVec2>) {
        self.selected_icon = value;
    }

    // Getter for `hovered_icon`
    pub fn get_hovered_icon(&self) -> Option<IVec2> {
        self.hovered_icon
    }

    // Setter for `hovered_icon`
    pub fn set_hovered_icon(&mut self, value: Option<IVec2>) {
        self.hovered_icon = value;
    }

    // Getter for `is_shown`
    pub fn is_shown(&self) -> bool {
        self.is_shown
    }

    // Setter for `is_shown`
    pub fn set_is_shown(&mut self, value: bool) {
        self.is_shown = value;
    }
}
