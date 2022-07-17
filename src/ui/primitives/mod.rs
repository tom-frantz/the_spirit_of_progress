use crate::ui::primitives::header::on_header_button_click;
use bevy::prelude::*;

pub mod header;

pub mod center_box;
pub mod sidebar;

pub struct UiPrimitivesPlugin;

impl Plugin for UiPrimitivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_header_button_click);
    }
}
