// use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::height::{HeightMap, HeightPoint};
use crate::components::world::latlon::LatLonPoint;
use crate::components::world::tectonics::plate::Plate;
use crate::components::world::tectonics::TectonicsMap;
use crate::ui::theme::{Colour, MenuColour};
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::{TileBundle, TileColor, TilePos};

#[derive(Debug, Clone)]
pub struct PlatePoint {
    pub(crate) plate_id: u32,
    age: u32,
}

impl PlatePoint {
    pub fn new(plate_id: u32, age: u32) -> Self {
        PlatePoint { plate_id, age }
    }

    pub fn tick(&mut self) {
        self.age += 1
    }
}
