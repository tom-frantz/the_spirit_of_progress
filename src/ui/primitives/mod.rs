use crate::ui::primitives::header::on_header_button_click;
use bevy::prelude::*;

pub mod header;
pub mod root;

pub mod center_box;
pub mod sidebar;

pub mod ui_box;

pub struct UiPrimitivesPlugin;

impl Plugin for UiPrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_header_button_click);
    }
}
