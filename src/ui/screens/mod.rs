use crate::ui::primitives::root::{RootElement, RootElementBundle};
use bevy::prelude::*;

pub mod weapon_design;

type ChangedScreenQuery<'world, 'state, 'q_component, ScreenType> = Query<
    'world,
    'state,
    (
        Entity,
        &'q_component Visibility,
        &'q_component mut ScreenType,
    ),
    Or<(Changed<ScreenType>, Changed<Visibility>)>,
>;

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
        screen_query: ChangedScreenQuery<Self>,
    ) {
        for (entity, visibility, self_component) in screen_query.iter() {
            if visibility.is_visible {
                let root_el_result = root_element_query.get_single();

                let mut root_el = {
                    if let Ok(parent_entity) = root_el_result {
                        commands.entity(parent_entity)
                    } else {
                        commands.spawn_bundle(RootElementBundle::new())
                    }
                };

                root_el.with_children(|child_builder| {
                    Self::draw(child_builder, &asset_server, entity, self_component)
                });
            }
        }
    }
}
