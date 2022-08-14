use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::WorldPoints;

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
