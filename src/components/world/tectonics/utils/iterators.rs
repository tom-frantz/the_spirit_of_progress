use crate::tectonics::utils::WorldTectonicsIndex;
use crate::{LatLonPoint, ValuePoint, WorldPoint, WorldTectonics};
use std::fmt::Debug;

#[derive(Debug)]
pub struct WorldTectonicsIterator<'a, T>
where
    T: Debug + Clone,
{
    world: &'a WorldTectonics<T>,
    current: Option<WorldTectonicsIndex>,
}

impl<'a, T> WorldTectonicsIterator<'a, T>
where
    T: Debug + Clone,
{
    pub fn new(world: &'a WorldTectonics<T>) -> Self {
        Self {
            world,
            current: Some(WorldTectonicsIndex::NorthPole),
        }
    }
}

#[derive(Debug)]
pub struct WorldTectonicsIntoIterator<T>
where
    T: Debug + Clone,
{
    world: WorldTectonics<T>,
    current: Option<WorldTectonicsIndex>,
}

impl<T> WorldTectonicsIntoIterator<T>
where
    T: Debug + Clone,
{
    pub fn new(world: WorldTectonics<T>) -> Self {
        Self {
            world,
            current: Some(WorldTectonicsIndex::NorthPole),
        }
    }
}

fn next_iter_point(current_point: WorldTectonicsIndex) -> Option<WorldTectonicsIndex> {
    match current_point {
        WorldTectonicsIndex::NorthPole => {
            Some(WorldTectonicsIndex::Point(LatLonPoint::new(89.5, -179.5)))
        }
        WorldTectonicsIndex::SouthPole => None,
        WorldTectonicsIndex::Point(point) => {
            let mut lat = point.lat();
            let mut lon = point.lon() + 0.5;

            if lon == 180.5 {
                lat -= 0.5;
                lon = -179.5;
            }

            Some(WorldTectonicsIndex::from(LatLonPoint::new(lat, lon)))
        }
    }
}

impl<'a, T> Iterator for WorldTectonicsIterator<'a, T>
where
    T: Debug + Clone,
{
    type Item = &'a ValuePoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(current_index) = self.current {
            let next = next_iter_point(current_index);

            let return_point: Option<&ValuePoint<T>> =
                self.current.map(|current_index| match current_index {
                    WorldTectonicsIndex::NorthPole => &self.world.north_pole_point,
                    WorldTectonicsIndex::SouthPole => &self.world.south_pole_point,
                    WorldTectonicsIndex::Point(point) => &self.world.points[&point],
                });

            self.current = next;
            return_point
        } else {
            None
        };
    }
}

impl<T> Iterator for WorldTectonicsIntoIterator<T>
where
    T: Debug + Clone,
{
    type Item = ValuePoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(current_index) = self.current {
            let next = next_iter_point(current_index);
            let return_point = self.current.map(|current_index| match current_index {
                WorldTectonicsIndex::NorthPole => self.world.north_pole_point.clone(),
                WorldTectonicsIndex::SouthPole => self.world.south_pole_point.clone(),
                WorldTectonicsIndex::Point(point) => self.world.points[&point].clone(),
            });

            self.current = next;
            return_point
        } else {
            None
        };
    }
}
