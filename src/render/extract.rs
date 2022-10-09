use crate::game::world::HexWorld;
use bevy::prelude::Query;

pub fn extract<T>(hex_world: Query<HexWorld<T>>) {
    println!("Hey this should be happening?! EXTRACT");
}
