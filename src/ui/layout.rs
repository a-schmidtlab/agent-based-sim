// UI layout management

// This module is currently simple - the layout is handled directly in the main app
// Could be extended for custom layouts, themes, etc.

pub struct UILayout {
    pub show_controls: bool,
    pub show_statistics: bool,
    pub control_panel_width: f32,
}

impl Default for UILayout {
    fn default() -> Self {
        Self {
            show_controls: true,
            show_statistics: true,
            control_panel_width: 300.0,
        }
    }
}
