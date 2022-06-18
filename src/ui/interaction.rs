use crate::ui::utils::get_cursor_location;
use crate::{City, Connection};
use bevy::ecs::entity::Entity;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MapClickable;

pub enum InteractionEvents {
    City(Entity),
    Connection(Entity),
}

pub trait Clickable {
    fn clicked(&self, transform: &Transform, mouse: Vec2) -> bool;

    fn event_type(&self, self_entity: Entity) -> InteractionEvents;
}

fn handle_clickable<T>(
    clickable_option: Option<&T>,
    transform: &Transform,
    mouse: Vec2,
    entity: Entity,
    mut map_interaction_events_writer: &mut EventWriter<InteractionEvents>,
) -> bool
where
    T: Clickable,
{
    if let Some(clickable) = clickable_option {
        if clickable.clicked(transform, mouse) {
            map_interaction_events_writer.send(clickable.event_type(entity));
            return true;
        }
    }
    false
}

pub fn click_event_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    q_clickable: Query<(
        Entity,
        &Transform,
        &MapClickable,
        Option<&City>,
        Option<&Connection>,
    )>,
    mut map_interaction_events_writer: EventWriter<InteractionEvents>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor_location = get_cursor_location(windows, q_camera).unwrap();
    for (entity, transform, _map_clickable, city_component, connection_component) in
        q_clickable.iter()
    {
        handle_clickable(
            city_component,
            transform,
            cursor_location,
            entity,
            &mut map_interaction_events_writer,
        );

        handle_clickable(
            connection_component,
            transform,
            cursor_location,
            entity,
            &mut map_interaction_events_writer,
        );
    }
}
