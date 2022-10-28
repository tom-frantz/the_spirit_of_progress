use crate::game::weapons::small_arms::SmallArmsPlugin;
use bevy::prelude::*;

pub mod component;
pub mod small_arms;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SmallArmsPlugin);
    }
}
