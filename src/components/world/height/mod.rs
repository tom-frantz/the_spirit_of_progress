use crate::components::world::latlon::ValuePoint;
use crate::components::world::render::{RenderTheWorld, World, WorldMap};
use bevy_ecs_tilemap::prelude::TileColor;
// use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::components::world::WorldPoints;
use crate::ui::theme::{Colour, Terrain};

#[derive(Debug, Clone)]
pub struct HeightPoint {
    pub height: f32,
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

impl<'a> World<'a> for HeightMap {
    type Point = HeightPoint;

    fn get_world(&self) -> &WorldPoints<Self::Point> {
        &self.world
    }
}

impl<'a> RenderTheWorld<'a> for HeightPoint {
    type World = HeightMap;

    fn colour(point: &<Self::World as WorldMap<'a>>::Point, world: &Self::World) -> TileColor {
        Terrain::SeaLevelLand.tile_color()
    }
}

impl<'a> IntoIterator for &'a HeightMap {
    type Item = &'a ValuePoint<HeightPoint>;
    type IntoIter = WorldPointsIterator<'a, HeightPoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}
