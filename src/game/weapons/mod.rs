use bevy::prelude::*;

mod component;
mod small_arms;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_toml);
    }
}

fn load_toml() {}
