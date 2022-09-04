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
        let colour = if point.height < 1. {
            Terrain::Sea6
        } else if point.height < 2. {
            Terrain::Sea5
        } else if point.height < 3. {
            Terrain::Sea4
        } else if point.height < 4. {
            Terrain::Sea3
        } else if point.height < 5. {
            Terrain::Sea2
        } else if point.height < 6. {
            Terrain::Sea1
        } else if point.height < 7. {
            Terrain::SeaLevelWater
        } else if point.height < 8. {
            Terrain::SeaLevelLand
        } else if point.height < 9. {
            Terrain::Land1
        } else if point.height < 10. {
            Terrain::Land2
        } else if point.height < 11. {
            Terrain::Land3
        } else if point.height < 12. {
            Terrain::Land4
        } else if point.height < 13. {
            Terrain::Land5
        } else if point.height < 14. {
            Terrain::Land6
        } else if point.height < 15. {
            Terrain::Land7
        } else if point.height < 16. {
            Terrain::Land8
        } else {
            Terrain::Land9
        };

        colour.tile_color()
    }
}

impl<'a> IntoIterator for &'a HeightMap {
    type Item = &'a ValuePoint<HeightPoint>;
    type IntoIter = WorldPointsIterator<'a, HeightPoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}
