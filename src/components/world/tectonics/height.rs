use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::WorldPoints;
use bevy::prelude::Color;
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
    type World = HeightMap;

    fn bundle(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileBundle {
        TileBundle {
            position,
            tilemap_id,
            color: TileColor(Color::rgb(1., 0., 0.)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    pub world: WorldPoints<HeightPoint>,
}

impl HeightMap {
    pub fn new(world: WorldPoints<HeightPoint>) -> Self {
        HeightMap { world }
    }
}

impl WorldRender for HeightMap {
    fn precision(&self) -> u32 {
        self.world.precision
    }
}
