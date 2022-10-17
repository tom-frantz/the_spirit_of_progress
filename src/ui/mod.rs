use crate::ui::primitives::sidebar::render_sidebar;
use crate::ui::primitives::UiPrimitivesPlugin;
use crate::ui::screens::weapon_design::{WeaponDesignMode, WeaponDesignScreen};
use crate::ui::screens::Screen;
use crate::ui::theme::{MenuColour, SPACING};
use bevy::prelude::Val::{Auto, Percent, Px};
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
pub enum RootElement {
    Sidebar,
    CenterBox,
}

#[derive(Component, Debug)]
pub struct RootComponent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_event::<MapInteractionEvents>()
            // .add_system(ui_click_event_consumer)
            // .add_system(click_event_generator)
            .add_startup_system(spawn_root_ui_node)
            .add_system(WeaponDesignScreen::on_change)
            .add_plugin(UiPrimitivesPlugin)
            // Debugs
            .add_startup_system(weapon_design_screen_debug);
    }
}

fn spawn_root_ui_node(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            color: UiColor::from(Color::NONE),

            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::all(Px(0.)),
                size: Size::new(Percent(100.), Percent(100.)),
                margin: UiRect::all(Px(0.)),
                flex_grow: 1.0,
                ..default()
            },
            ..Default::default()
        })
        .insert(RootComponent);
}

fn weapon_design_screen_debug(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle::default())
        .insert(WeaponDesignScreen::default())
        .insert(Visibility { is_visible: true });
}
