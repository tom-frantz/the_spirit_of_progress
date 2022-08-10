use crate::tectonics::utils::WorldTectonicsIndex;
use crate::{LatLonPoint, ValuePoint, WorldPoint, WorldPoints};
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum WorldPointsIterCursor {
    NorthPole,
    SouthPole,
    Point(usize),
}

#[derive(Debug)]
pub struct WorldTectonicsIterator<'a, T>
where
    T: Debug + Clone,
{
    world: &'a WorldPoints<T>,
    cursor: Option<WorldPointsIterCursor>,
}

impl<'a, T> WorldTectonicsIterator<'a, T>
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
pub struct WorldTectonicsIntoIterator<T>
where
    T: Debug + Clone,
{
    world: WorldPoints<T>,
    cursor: Option<WorldPointsIterCursor>,
}

impl<T> WorldTectonicsIntoIterator<T>
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

impl<'a, T> Iterator for WorldTectonicsIterator<'a, T>
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

impl<T> Iterator for WorldTectonicsIntoIterator<T>
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
    type IntoIter = WorldTectonicsIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        WorldTectonicsIntoIterator::new(self)
    }
}

impl<'a, T> IntoIterator for &'a WorldPoints<T>
where
    T: Debug + Clone,
{
    type Item = &'a ValuePoint<T>;
    type IntoIter = WorldTectonicsIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        WorldTectonicsIterator::new(self)
    }
}
