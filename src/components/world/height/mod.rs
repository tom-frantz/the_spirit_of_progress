use crate::components::world::latlon::ValuePoint;
// use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::utils::iterators::WorldPointsIterator;
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

#[derive(Debug, Clone)]
pub struct HeightMap {
    pub world: WorldPoints<HeightPoint>,
}

impl HeightMap {
    pub fn new(world: WorldPoints<HeightPoint>) -> Self {
        HeightMap { world }
    }
}

// impl WorldRender for HeightMap {
//     fn precision(&self) -> u32 {
//         self.world.precision
//     }
// }
//
// impl TileRender for HeightPoint {
//     type World = HeightMap;
// }

impl<'a> IntoIterator for &'a HeightMap {
    type Item = &'a ValuePoint<HeightPoint>;
    type IntoIter = WorldPointsIterator<'a, HeightPoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}
