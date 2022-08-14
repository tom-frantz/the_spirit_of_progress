use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImageSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use vads::camera::CameraPlugin;
use vads::components::world::latlon::{LATITUDE_RANGE, LONGITUDE_RANGE};
use vads::components::world::render::draw_map;
use vads::components::world::tectonics::TectonicPlates;
use vads::components::world::{PIXEL_BUFFER, TECTONIC_PRECISION};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "World Simulation".to_string(),
            width: (LONGITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION as f32 * 3.,
            height: (LATITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION as f32 * 3.,

            ..default()
        })
        .insert_resource(WgpuSettings {
            backends: Some(Backends::DX12),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(UiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TilemapPlugin)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(draw_height_map)
        // .add_system(delete_all)
        // .add_startup_system(render::sanity_check)
        .run()
}

fn draw_height_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world = TectonicPlates::new(2, 2, 6);

    draw_map(&world, commands, asset_server)
}

// fn delete_all(
//     mut commands: Commands,
//     keyboard_input: Res<Input<KeyCode>>,
//     tile_maps: Query<Entity, With<MainTileMap>>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Back) {
//         for tile_map in tile_maps.iter() {
//             commands.entity(tile_map).despawn_recursive();
//         }
//     }
// }
