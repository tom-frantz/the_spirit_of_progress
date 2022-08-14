use crate::components::world::height::HeightPoint;
use crate::components::world::latlon::ValuePoint;
use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::PlateType;
use crate::components::world::tectonics::TectonicPlates;
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::ui::theme::{Colour, MenuColour, Terrain};
use bevy::asset::AssetServer;
use bevy::prelude::{Res, Transform};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTileSize};
use bevy_ecs_tilemap::prelude::{TileBundle, TileColor, TilePos, TileStorage};
use bevy_ecs_tilemap::TilemapBundle;

impl WorldRender for TectonicPlates {
    fn precision(&self) -> u32 {
        self.world.precision
    }
}

impl TileRender for PlatePoint {
    type World = TectonicPlates;

    fn color(&self, world: &Self::World, position: TilePos, _tilemap_id: TilemapId) -> TileColor {
        let mut color = TileColor(world.plates[&self.plate_id].colour);

        for (_id, plate) in &world.plates {
            let origin_pos = plate.origin.tile_pos(world.precision());
            if origin_pos.x == position.x && origin_pos.y == position.y {
                color = MenuColour::BlackPen.tile_color()
            }
        }

        color
    }
}
