use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::tectonics::TectonicPlates;
use crate::ui::theme::{Colour, MenuColour};
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::{TileBundle, TileColor, TilePos};

#[derive(Debug, Clone)]
pub struct PlatePoint {
    pub(crate) plate_id: u32,
    strength: f32,
}

impl PlatePoint {
    pub fn new(plate_id: u32, strength: f32) -> Self {
        PlatePoint { plate_id, strength }
    }
}
