use crate::ui::RootElement;
use bevy::prelude::*;

pub mod weapon_design;

pub trait Screen
where
    Self: Component + Sized + Send,
{
    fn draw(
        commands: &mut Commands,
        asset_server: &AssetServer,
        parent: &mut ChildBuilder,
        entity: Entity,
        component: &Self,
    );

    fn on_change(
        mut commands: Commands,
        asset_server: Res<AssetServer>,

        root_element_query: Query<(Entity), (&RootElement)>,
        screen_query: Query<
            (Entity, &Visibility, &mut Self),
            (Or<(Changed<Self>, Changed<Visibility>)>),
        >,
    ) {
        for (entity, visibility, self_component) in screen_query.iter() {
            println!("HEY! THIS SHOULD BE CALLED");
            if visibility.is_visible {
                let root_el = root_element_query.single();

                Self::draw(
                    &mut commands,
                    &asset_server,
                    root_el,
                    entity,
                    self_component,
                )
            }
        }
    }
}
