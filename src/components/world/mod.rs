use crate::latlon::{LatLonPoint, ValuePoint, WorldPoint, LATITUDE_RANGE, LONGITUDE_RANGE};
use crate::render::draw_map;
use crate::tectonics::plates::PlatePoint;
use crate::tectonics::utils::WorldTectonicsIndex;
use crate::tectonics::WorldPoints;
use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub mod latlon;
pub mod render;
pub mod tectonics;

const PIXEL_SIZE: f32 = 3.;
const PIXEL_BUFFER: f32 = 5.;
const TECTONIC_PRECISION: u32 = 2;

fn main() {
    App::new()
        .insert_resource(WgpuSettings {
            backends: Some(Backends::DX12),
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: (LONGITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION as f32 * PIXEL_SIZE,
            height: (LATITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION as f32 * PIXEL_SIZE,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(TilemapPlugin)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(draw_height_map)
        .run()
}

fn draw_height_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world = WorldPoints::new(2, |point| PlatePoint::new(0, 0.));

    draw_map(world, commands, asset_server)
}
