use crate::components::world::height::{HeightMap, HeightPoint};
use crate::components::world::latlon::ValuePoint;
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::TectonicPlates;
use crate::components::world::WorldPoints;
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum WorldPointsIterCursor {
    NorthPole,
    SouthPole,
    Point(usize),
}

#[derive(Debug)]
pub struct WorldPointsIterator<'a, T>
where
    T: Debug + Clone,
{
    world: &'a WorldPoints<T>,
    cursor: Option<WorldPointsIterCursor>,
}

impl<'a, T> WorldPointsIterator<'a, T>
where
    T: Debug + Clone,
{
    pub fn new(world: &'a WorldPoints<T>) -> Self {
        Self {
            world,
            cursor: Some(WorldPointsIterCursor::NorthPole),
        }
    }
}

#[derive(Debug)]
pub struct WorldPointsIntoIterator<T>
where
    T: Debug + Clone,
{
    world: WorldPoints<T>,
    cursor: Option<WorldPointsIterCursor>,
}

impl<T> WorldPointsIntoIterator<T>
where
    T: Debug + Clone,
{
    pub fn new(world: WorldPoints<T>) -> Self {
        Self {
            world,
            cursor: Some(WorldPointsIterCursor::NorthPole),
        }
    }
}

fn next_from_cursor(
    current_cursor: &WorldPointsIterCursor,
    points_len: usize,
) -> Option<WorldPointsIterCursor> {
    match current_cursor {
        WorldPointsIterCursor::NorthPole => Some(WorldPointsIterCursor::Point(0)),
        WorldPointsIterCursor::SouthPole => None,
        WorldPointsIterCursor::Point(index) => {
            if index + 1 == points_len {
                Some(WorldPointsIterCursor::SouthPole)
            } else {
                Some(WorldPointsIterCursor::Point(index + 1))
            }
        }
    }
}

impl<'a, T> Iterator for WorldPointsIterator<'a, T>
where
    T: Debug + Clone,
{
    type Item = &'a ValuePoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(current_cursor) = &self.cursor {
            let next = next_from_cursor(current_cursor, self.world.points_len());

            let return_point: &ValuePoint<T> = match current_cursor {
                WorldPointsIterCursor::NorthPole => &self.world.north_pole_point,
                WorldPointsIterCursor::SouthPole => &self.world.south_pole_point,
                WorldPointsIterCursor::Point(index) => &self.world.points[*index],
            };

            self.cursor = next;
            Some(return_point)
        } else {
            None
        };
    }
}

impl<T> Iterator for WorldPointsIntoIterator<T>
where
    T: Debug + Clone,
{
    type Item = ValuePoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(current_cursor) = &self.cursor {
            let next: Option<WorldPointsIterCursor> =
                next_from_cursor(current_cursor, self.world.points_len());

            let return_point: ValuePoint<T> = match current_cursor {
                WorldPointsIterCursor::NorthPole => self.world.north_pole_point.clone(),
                WorldPointsIterCursor::SouthPole => self.world.south_pole_point.clone(),
                WorldPointsIterCursor::Point(_index) => self.world.points.remove(0),
            };

            self.cursor = next;
            Some(return_point)
        } else {
            None
        };
    }
}

impl<T> IntoIterator for WorldPoints<T>
where
    T: Debug + Clone,
{
    type Item = ValuePoint<T>;
    type IntoIter = WorldPointsIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIntoIterator::new(self)
    }
}

impl<'a, T> IntoIterator for &'a WorldPoints<T>
where
    T: Debug + Clone,
{
    type Item = &'a ValuePoint<T>;
    type IntoIter = WorldPointsIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a TectonicPlates {
    type Item = &'a ValuePoint<PlatePoint>;
    type IntoIter = WorldPointsIterator<'a, PlatePoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}

impl<'a> IntoIterator for &'a HeightMap {
    type Item = &'a ValuePoint<HeightPoint>;
    type IntoIter = WorldPointsIterator<'a, HeightPoint>;

    fn into_iter(self) -> Self::IntoIter {
        WorldPointsIterator::new(&self.world)
    }
}
