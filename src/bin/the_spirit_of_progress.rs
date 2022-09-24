use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(TilemapPlugin)
        .insert_resource(WgpuSettings {
            backends: Some(bevy::render::settings::Backends::DX12),
            ..Default::default()
        })
        .run();
}
