use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_prototype_lyon::prelude::*;
use vads::camera::CameraPlugin;
use vads::components::city::City;
use vads::components::connection::Connection;
use vads::components::world::render::plugin::WorldRenderPlugin;
use vads::map::create_map;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldRenderPlugin)
        .insert_resource(WgpuSettings {
            backends: Some(bevy::render::settings::Backends::DX12),
            ..Default::default()
        })
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(create_map());

    let milbourne = commands
        .spawn_bundle(City::new("Milbourne".to_string(), Vec2::new(400.0, 100.0)))
        .id();

    let moodend = commands
        .spawn_bundle(City::new("Moodend".to_string(), Vec2::new(200.0, 200.0)))
        .id();

    commands.spawn_bundle(Connection::new(
        (milbourne, Transform::from_xyz(400.0, 100.0, 0.0)),
        (moodend, Transform::from_xyz(200.0, 200.0, 0.0)),
    ));
}
