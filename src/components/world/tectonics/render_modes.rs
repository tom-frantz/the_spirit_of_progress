use crate::components::world::latlon::ValuePoint;
use crate::components::world::render::{RenderTheWorld, World, WorldMap};
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::{PlateType, TectonicPlates};
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::ui::theme::{Colour, Terrain};
use bevy::prelude::Transform;
use bevy_ecs_tilemap::prelude::*;

impl<'a> RenderTheWorld<'a> for PlatePoint {
    type World = TectonicPlates;

    fn colour(point: &<Self::World as WorldMap<'a>>::Point, world: &Self::World) -> TileColor {
        TileColor(world.plates[&point.value.plate_id].colour)
    }
}

impl<'a> RenderTheWorld<'a> for PlateType {
    type World = TectonicPlates;

    fn colour(point: &<Self::World as WorldMap<'a>>::Point, world: &Self::World) -> TileColor {
        match world.plates[&point.value.plate_id].plate_type {
            PlateType::Oceanic => TileColor(Terrain::SeaLevelWater.color()),
            PlateType::Continental => TileColor(Terrain::SeaLevelLand.color()),
        }
    }
}
