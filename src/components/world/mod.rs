use crate::components::world::render::draw_map;
use crate::components::world::tectonics::plates::PlatePoint;
use crate::components::world::tectonics::WorldPoints;
use bevy::prelude::*;

pub mod latlon;
pub mod render;
pub mod tectonics;

pub const PIXEL_SIZE: f32 = 10.;
pub const PIXEL_BUFFER: f32 = 5.;
pub const TECTONIC_PRECISION: u32 = 2;

fn draw_height_map(commands: Commands, asset_server: Res<AssetServer>) {
    let world = WorldPoints::new(2, |_point| PlatePoint::new(0, 0.));

    draw_map(world, commands, asset_server)
}
