use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImageSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use the_spirit_of_progress::render::RenderPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Hexworld Shaders".to_string(),
            width: 900.,
            height: 900.,

            ..default()
        })
        .insert_resource(WgpuSettings {
            backends: Some(Backends::DX12),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(RenderPlugin)
        .insert_resource(ImageSettings::default_nearest())
        // .insert_plugin()
        .run()
}
