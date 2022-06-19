use self::MainElements::*;
use crate::components::city::CityComponents;
use crate::ui::components::city_info::create_city_info;
use crate::ui::components::{render_root_ui, RootNode};
use crate::ui::interaction::MapInteractionEvents::*;
use crate::ui::interaction::{click_event_generator, MapInteractionEvents};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub mod components;
pub mod fonts;
pub mod interaction;
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
    CityInfo,
    ConnectionInfo,
}

fn clear_ui(mut commands: &mut Commands, q_ui_main_elements: &Query<Entity, With<MainElements>>) {
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
        println!("Received Event: {:?}", *map_event);
        match map_event {
            City(entity) => {
                println!("Doing this?!");
                clear_ui(&mut commands, &q_ui_main_elements);

                commands.entity(root_node).with_children(|parent| {
                    create_city_info(*entity, &q_city_ui_components, &asset_server, parent);
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
            .add_system(click_event_generator);
    }
}
