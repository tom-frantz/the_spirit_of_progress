use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImageSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_prototype_lyon::prelude::*;
use vads::camera::CameraPlugin;
use vads::components::world::latlon::{LATITUDE_RANGE, LONGITUDE_RANGE};
use vads::components::world::render::draw_map;
use vads::components::world::tectonics::height::{HeightMap, HeightPoint};
use vads::components::world::tectonics::plates::{PlatePoint, TectonicPlates};
use vads::components::world::tectonics::WorldPoints;
use vads::components::world::{render, PIXEL_BUFFER, PIXEL_SIZE, TECTONIC_PRECISION};

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
        // .add_startup_system(render::sanity_check)
        .run()
}

fn draw_height_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world = HeightMap::new(WorldPoints::new(2, |_p| HeightPoint::new(0.)));

    draw_map(world, commands, asset_server)
}
