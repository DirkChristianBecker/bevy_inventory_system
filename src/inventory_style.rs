use bevy::prelude::*;

#[derive(Resource)]
pub struct InventoryStyle {
    rows: u64,
    columns: u64,

    // Border color
    normal_border_color: Color,
    hover_border_color: Color,
    selected_border_color: Color,

    icon_size: f32,
    icon_margin: f32,
    icon_padding: f32,
    border_width: f32,

    // Background color
    normal_background_color: Color,
    hover_background_color: Color,
    selected_background_color: Color,

    // Fonts
    default_font: String, // Path to the font file
    default_font_size: f32,
    default_font_color: Color,

    // Positioning
    positioning: Val,
}

impl InventoryStyle {
    pub fn get_grid_size(&self) -> usize {
        let r = self.icon_size;

        r as usize
    }

    pub fn get_positioning(&self) -> Val { self.positioning }
    pub fn get_icon_size(&self) -> f32  { self.icon_size }
    pub fn get_icon_padding(&self) -> f32  { self.icon_padding }
    pub fn get_icon_margin(&self) -> f32  { self.icon_margin }
    
    pub fn get_border_with(&self) -> f32  { self.border_width }

    pub fn get_rows(&self) -> u64  { self.rows }
    pub fn get_columns(&self) -> u64  { self.columns }

    pub fn get_normal_font(&self) -> &String { &self.default_font }
    pub fn get_normal_font_size(&self) -> f32 { self.default_font_size }
    pub fn get_normal_font_color(&self) -> &Color { &self.default_font_color }

    pub fn get_selected_background_color(&self) -> &Color { &self.selected_background_color }
    pub fn get_selected_border_color(&self) -> &Color { &self.selected_border_color }

    pub fn get_hover_background_color(&self) -> &Color { &self.hover_background_color }
    pub fn get_hover_border_color(&self) -> &Color { &self.hover_border_color }

    pub fn get_normal_background_color(&self) -> &Color { &self.normal_background_color }
    pub fn get_normal_border_color(&self) -> &Color { &self.normal_border_color }

    pub fn get_total_height(&self) -> usize {
        self.get_grid_size() * self.rows as usize
    }

    pub fn get_total_width(&self) -> usize {
        self.get_grid_size() * self.columns as usize
    }
}

impl Default for InventoryStyle {
    fn default() -> Self {
        Self {
            rows: 4,
            columns: 6,
            normal_border_color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            hover_border_color: Color::GRAY,
            selected_border_color: Color::GRAY,

            icon_size: 64.0,
            icon_margin: 2.5,
            icon_padding: 2.5,
            border_width: 2.0,

            normal_background_color: Color::rgba(1.0, 1.0, 1.0, 0.5), // White
            hover_background_color: Color::rgba(1.0, 1.0, 1.0, 0.5),  // White
            selected_background_color: Color::rgba(1.0, 1.0, 1.0, 0.75), // White

            default_font: "Apocalypse/HUD/Fonts/SairaCondensed/SairaCondensed-Regular.ttf".to_string(),
            default_font_size: 24.0,
            default_font_color: Color::GRAY,

            positioning: Val::Auto,
        }
    }
}