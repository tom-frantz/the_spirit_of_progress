use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImageSettings;
use bevy::ui::UiPlugin;
use bevy_ecs_tilemap::TilemapPlugin;
use vads::camera::CameraPlugin;
use vads::components::world::latlon::{LATITUDE_RANGE, LONGITUDE_RANGE};
use vads::components::world::render::plugin::WorldRenderPlugin;
use vads::components::world::render::resources::GeographicWorld;
use vads::components::world::render::{MainTileMap, RenderTheWorld};
use vads::components::world::tectonics::plate::PlateType;
use vads::components::world::tectonics::point::PlatePoint;
use vads::components::world::tectonics::TectonicsMap;
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
        .add_plugin(WorldRenderPlugin)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(draw_height_map)
        // .add_startup_system(render::sanity_check)
        .run()
}

fn draw_height_map(
    geo_world: Res<GeographicWorld>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    tile_maps: Query<Entity, With<MainTileMap>>,
) {
    geo_world.draw_world_type(commands, asset_server, tile_maps);
}
