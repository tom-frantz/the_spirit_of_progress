use crate::{
    game::world::{tectonics::WorldTectonicsData, HexWorld, HexWorldData},
    render::traits::QueryCellRender,
};
use bevy::prelude::*;
use bevy::render::Extract;

pub fn extract(hex_world: Extract<Query<(Entity, &HexWorld, &WorldTectonicsData)>>) {
    println!("Hey this should be happening?! EXTRACT");

    for (entity, hex_world, tectonics_data) in hex_world.iter() {
        println!("{entity:?} {hex_world:?} {tectonics_data:?}")
    }
}
