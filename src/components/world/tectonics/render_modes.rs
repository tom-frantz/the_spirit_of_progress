use crate::components::world::height::HeightPoint;
use crate::components::world::latlon::ValuePoint;
// use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::PlateType;
use crate::components::world::tectonics::TectonicPlates;
use crate::components::world::utils::iterators::{WorldPointsIntoIterator, WorldPointsIterator};
use crate::ui::theme::{Colour, MenuColour, Terrain};
use bevy::asset::AssetServer;
use bevy::prelude::{Color, Res, Transform};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTileSize};
use bevy_ecs_tilemap::prelude::{TileBundle, TileColor, TilePos, TileStorage};
use bevy_ecs_tilemap::TilemapBundle;
use std::iter::Map;

// impl WorldRender for TectonicPlates {
//     fn precision(&self) -> u32 {
//         self.world.precision
//     }
// }
//
// impl TileRender for PlatePoint {
//     type World = TectonicPlates;
//
//     fn color(&self, world: &Self::World, position: TilePos, _tilemap_id: TilemapId) -> TileColor {
//         let mut color = TileColor(world.plates[&self.plate_id].colour);
//
//         for (_id, plate) in &world.plates {
//             let origin_pos = plate.origin.tile_pos(world.precision());
//             if origin_pos.x == position.x && origin_pos.y == position.y {
//                 color = MenuColour::BlackPen.tile_color()
//             }
//         }
//
//         color
//     }
// }

impl<'a> IntoIterator for &'a TectonicPlates {
    type Item = &'a ValuePoint<PlatePoint>;
    type IntoIter = WorldPointsIterator<'a, PlatePoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}

#[derive(Debug, Clone)]
pub struct TectonicPlatesTypes<'a>(pub &'a TectonicPlates);
#[derive(Debug, Clone)]
pub struct PlateTypePoint<'a>(pub &'a ValuePoint<PlatePoint>);

pub struct TectonicPlatesTypesIterator<'a> {
    tectonic_plates_types: &'a TectonicPlatesTypes<'a>,
    world_iter: WorldPointsIterator<'a, PlatePoint>,
}

impl<'a> TectonicPlatesTypesIterator<'a> {
    pub fn new(tectonic_plates_types: &'a TectonicPlatesTypes) -> Self {
        TectonicPlatesTypesIterator {
            tectonic_plates_types,
            world_iter: tectonic_plates_types.0.world.iter(),
        }
    }
}

// impl<'a> Iterator for TectonicPlatesTypesIterator<'a> {
//     type Item = &'a PlateTypePoint<'a>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.world_iter.next()
//     }
// }

// impl<'a> WorldRender for TectonicPlatesTypes<'a> {
//     fn precision(&self) -> u32 {
//         self.0.precision()
//     }
// }
//
// impl<'a> TileRender for PlateTypePoint<'a> {
//     type World = TectonicPlatesTypes<'a>;
//
//     fn color(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileColor {
//         TileColor(Color::rgb(1.0, 0.0, 0.0))
//     }
// }
//
// impl<'a> IntoIterator for &'a TectonicPlatesTypes<'a> {
//     type Item = &'a PlateTypePoint<'a>;
//     type IntoIter = TectonicPlatesTypesIterator<'a>;
//
//     fn into_iter(self: &'a TectonicPlatesTypes<'a>) -> Self::IntoIter {
//         TectonicPlatesTypesIterator::new(self)
//     }
// }
