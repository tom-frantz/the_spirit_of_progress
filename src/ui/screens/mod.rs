use bevy::prelude::*;

pub mod weapon_design;

pub trait Screen
where
    Self: Component + Sized + Send,
{
    fn draw(commands: &mut Commands, asset_server: &AssetServer, entity: Entity, component: &Self);

    fn on_change(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        query: Query<(Entity, &Visibility, &mut Self), (Changed<Self>, Changed<Visibility>)>,
    ) {
        for (entity, visibility, self_component) in query.iter() {
            if visibility.is_visible {
                Self::draw(&mut commands, &asset_server, entity, self_component)
            }
        }
    }
}
