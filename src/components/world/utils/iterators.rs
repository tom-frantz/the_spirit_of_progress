use crate::components::world::height::{HeightMap, HeightPoint};
use crate::components::world::latlon::ValuePoint;
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::TectonicsMap;
use crate::components::world::WorldPoints;
use bevy::ui::Val;
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

pub struct MutWorldPointsIterator<'a, T>
where
    T: Debug + Clone,
{
    world: &'a mut WorldPoints<T>,
    cursor: Option<WorldPointsIterCursor>,
}

impl<'a, T> MutWorldPointsIterator<'a, T>
where
    T: Debug + Clone + 'a,
{
    pub fn new(world: &'a mut WorldPoints<T>) -> Self {
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

impl<'iterator, T> Iterator for MutWorldPointsIterator<'iterator, T>
where
    T: Debug + Clone + 'iterator,
{
    type Item = &'iterator mut ValuePoint<T>;

    fn next(self: &'_ mut MutWorldPointsIterator<'iterator, T>) -> Option<Self::Item> {
        return if let Some(current_cursor) = &mut self.cursor {
            let next = next_from_cursor(current_cursor, self.world.points_len());

            // Better be damn sure I don't repeat a lookup c;
            // https://stackoverflow.com/questions/61978903/how-do-i-create-mutable-iterator-over-struct-fields
            let mut return_point: &'iterator mut ValuePoint<T> = unsafe {
                match current_cursor {
                    WorldPointsIterCursor::NorthPole => {
                        &mut *(&mut self.world.north_pole_point as *mut _)
                    }
                    WorldPointsIterCursor::SouthPole => {
                        &mut *(&mut self.world.south_pole_point as *mut _)
                    }
                    WorldPointsIterCursor::Point(index) => {
                        &mut *(&mut self.world.points[*index] as *mut _)
                    }
                }
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

impl<'a, T> IntoIterator for &'a mut WorldPoints<T>
where
    T: Debug + Clone,
{
    type Item = &'a mut ValuePoint<T>;
    type IntoIter = MutWorldPointsIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MutWorldPointsIterator::new(self)
    }
}
