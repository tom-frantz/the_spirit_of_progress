use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        texture::ImageSettings,
    },
    ui::UiPlugin,
};
use bevy_ecs_tilemap::TilemapPlugin;
use the_spirit_of_progress::camera::CameraPlugin;
use the_spirit_of_progress::{
    game::world::{HexWorld, HexWorldData},
    render::RenderPlugin,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Hexworld Shaders".to_string(),
            width: 900.,
            height: 900.,

            ..default()
        })
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WgpuSettings {
            backends: Some(Backends::DX12),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CameraPlugin)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(init)
        // .insert_plugin()
        .run()
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(HexWorld::new());
}
