use crate::camera::MainCamera;
use crate::components::city::City;
use crate::components::connection::Connection;
use crate::ui::utils::get_cursor_location;
use bevy::ecs::entity::Entity;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MapClickable;

#[derive(Debug)]
pub enum MapInteractionEvents {
    City(Entity),
    Connection(Entity),
}

pub trait Clickable {
    fn clicked(&self, transform: &Transform, mouse: Vec2) -> bool;

    fn event_type(&self, self_entity: Entity) -> MapInteractionEvents;
}

pub fn click_event_generator(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, With<MainCamera>)>,
    mouse_buttons: Res<Input<MouseButton>>,
    q_clickable: Query<(
        Entity,
        &Transform,
        &MapClickable,
        Option<&City>,
        Option<&Connection>,
    )>,
    mut map_interaction_events_writer: EventWriter<MapInteractionEvents>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let mouse = get_cursor_location(windows, q_camera).unwrap();
    for (entity, transform, _map_clickable, city_component, connection_component) in
        q_clickable.iter()
    {
        if let Some(clickable) = city_component {
            if clickable.clicked(transform, mouse) {
                map_interaction_events_writer.send(clickable.event_type(entity));
                // This return (and all following) guarantees one event will fire per tick
                return;
            }
        } else if let Some(clickable) = connection_component {
            if clickable.clicked(transform, mouse) {
                map_interaction_events_writer.send(clickable.event_type(entity));
                return;
            }
        } else {
            panic!("[1] An entity with a MapClickable component did not have an associated Clickable impl.");
        }
    }
}
