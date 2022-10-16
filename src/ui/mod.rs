use crate::ui::primitives::sidebar::render_sidebar;
use crate::ui::primitives::UiPrimitivesPlugin;
use crate::ui::screens::weapon_design::{WeaponDesignMode, WeaponDesignScreen};
use crate::ui::screens::Screen;
use bevy::prelude::*;

mod screens;

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

#[derive(Bundle)]
pub struct LabelledButtonBundle<Label: Component> {
    #[bundle]
    node_bundle: ButtonBundle,
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
            // .add_system(ui_click_event_consumer)
            // .add_system(click_event_generator)
            .add_startup_system(weapon_design_screen_debug)
            .add_system(WeaponDesignScreen::on_change)
            .add_plugin(UiPrimitivesPlugin);
    }
}

fn weapon_design_screen_debug(mut commands: Commands) {
    commands.spawn().insert(WeaponDesignScreen::default());
}
