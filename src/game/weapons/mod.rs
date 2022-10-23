use bevy::prelude::*;

mod components;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_toml);
    }
}

fn load_toml() {}
