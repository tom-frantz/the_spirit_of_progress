use crate::ui::{primitives::root::get_root_node_bundle, RootElement};
use bevy::prelude::*;

pub mod weapon_design;

pub trait Screen
where
    Self: Component + Sized + Send,
{
    fn draw(
        parent: &mut ChildBuilder,
        asset_server: &AssetServer,
        entity: Entity,
        component: &Self,
    );

    fn on_change(
        mut commands: Commands,
        asset_server: Res<AssetServer>,

        root_element_query: Query<Entity, &RootElement>,
        screen_query: Query<
            (Entity, &Visibility, &mut Self),
            Or<(Changed<Self>, Changed<Visibility>)>,
        >,
    ) {
        for (entity, visibility, self_component) in screen_query.iter() {
            println!("HEY! THIS SHOULD BE CALLED");
            if visibility.is_visible {
                let root_el_result = root_element_query.get_single();

                let mut root_el = {
                    if let Ok(parent_entity) = root_el_result {
                        commands.entity(parent_entity)
                    } else {
                        commands.spawn_bundle(get_root_node_bundle())
                    }
                };

                root_el.with_children(|child_builder| {
                    Self::draw(child_builder, &asset_server, entity, self_component)
                });
            }
        }
    }
}
