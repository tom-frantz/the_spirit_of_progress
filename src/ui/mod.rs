use self::MainElements::*;
use crate::components::city::CityComponents;
// use crate::ui::components::city_info::{create_city_info, CityInfo};
use crate::ui::components::city_info::render_city_info;
use crate::ui::components::{render_root_ui, RootNode};
use crate::ui::fonts::Typography;
use crate::ui::interaction::MapInteractionEvents::*;
use crate::ui::interaction::{click_event_generator, MapInteractionEvents};
use crate::ui::primitives::header::on_header_button_click;
use crate::ui::primitives::sidebar::{render_sidebar, SIDEBAR_CONTENT_SIZE};
use bevy::prelude::Val::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub mod components;
pub mod fonts;
pub mod interaction;
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

pub fn clear_ui_elements(
    mut commands: &mut Commands,
    q_ui_main_elements: &Query<Entity, With<MainElements>>,
) {
    for entity in q_ui_main_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn ui_click_event_consumer(
    mut map_interaction_events: EventReader<MapInteractionEvents>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,

    q_ui_root_node: Query<Entity, With<RootNode>>,
    q_ui_main_elements: Query<Entity, With<MainElements>>,

    q_city_ui_components: CityComponents,
) {
    let root_node = q_ui_root_node.single();

    for map_event in map_interaction_events.iter() {
        clear_ui_elements(&mut commands, &q_ui_main_elements);

        match map_event {
            City(entity) => {
                let city = q_city_ui_components.get(*entity).unwrap();

                commands.entity(root_node).with_children(|parent| {
                    render_sidebar(parent, |sidebar_content| {
                        render_city_info(sidebar_content, &asset_server, city);
                    });
                });
            }
            Connection(entity) => {}
        }
    }
}

fn create_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapInteractionEvents>()
            .add_startup_system(create_ui_camera)
            .add_startup_system(render_root_ui)
            .add_system(ui_click_event_consumer)
            .add_system(click_event_generator)
            .add_system(on_header_button_click);
    }
}
