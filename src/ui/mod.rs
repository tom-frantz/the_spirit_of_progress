use crate::ui::primitives::sidebar::render_sidebar;
use crate::ui::primitives::UiPrimitivesPlugin;
use bevy::prelude::*;

pub mod fonts;
pub mod primitives;
pub mod theme;
mod utils;

#[derive(Bundle)]
pub struct LabelledNodeBundle<Label: Component> {
    #[bundle]
    node_bundle: NodeBundle,
    label: Label,
}

#[derive(Bundle)]
pub struct LabelledTextBundle<Label: Component> {
    #[bundle]
    node_bundle: TextBundle,
    label: Label,
}

#[derive(Component, Debug)]
pub enum MainElements {
    Sidebar,
    CenterBox,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<MapInteractionEvents>()
            // .add_startup_system(render_root_ui)
            // .add_system(ui_click_event_consumer)
            // .add_system(click_event_generator)
            .add_plugin(UiPrimitivesPlugin);
    }
}
