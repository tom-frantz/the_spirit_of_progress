use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        texture::ImageSettings,
    },
    ui::UiPlugin,
};
use bevy_ecs_tilemap::TilemapPlugin;
use the_spirit_of_progress::{
    camera::CameraPlugin,
    game::world::{HexWorld, HexWorldData},
    render::RenderPlugin,
    SpiritOfProgressPlugin,
};

fn main() {
    App::new()
        .add_plugin(SpiritOfProgressPlugin)
        .add_startup_system(init)
        // .insert_plugin()
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(HexWorld::new());
}
