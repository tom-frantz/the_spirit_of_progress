use crate::components::world::render::TileRender;
use crate::components::world::WorldPoints;
use bevy_ecs_tilemap::prelude::*;

#[derive(Debug, Clone)]
pub struct HeightPoint {
    height: f32,
}

impl HeightPoint {
    pub fn new(height: f32) -> Self {
        HeightPoint { height }
    }
}

impl TileRender for HeightPoint {
    fn bundle(&self, position: TilePos, tilemap_id: TilemapId) -> TileBundle {
        TileBundle {
            position,
            tilemap_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    world: WorldPoints<HeightPoint>,
}

impl HeightMap {
    pub fn new(world: WorldPoints<HeightPoint>) -> Self {
        HeightMap { world }
    }
}
