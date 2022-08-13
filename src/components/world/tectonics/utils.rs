use crate::components::world::tectonics::{LatLonPoint, ValuePoint, WorldPoint};
use std::fmt::Debug;

pub mod iterators;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WorldTectonicsIndex {
    NorthPole,
    SouthPole,
    Point(LatLonPoint),
}

impl<T> From<ValuePoint<T>> for WorldTectonicsIndex
where
    T: Debug + Clone,
{
    fn from(value_point: ValuePoint<T>) -> Self {
        value_point.point.into()
    }
}

impl From<LatLonPoint> for WorldTectonicsIndex {
    fn from(value_point: LatLonPoint) -> Self {
        if value_point.lat() == 90. {
            WorldTectonicsIndex::NorthPole
        } else if value_point.lat() == -90. {
            WorldTectonicsIndex::SouthPole
        } else {
            WorldTectonicsIndex::Point(value_point)
        }
    }
}

impl From<WorldTectonicsIndex> for LatLonPoint {
    fn from(index: WorldTectonicsIndex) -> Self {
        match index {
            WorldTectonicsIndex::NorthPole => LatLonPoint::new(90., 0.),
            WorldTectonicsIndex::SouthPole => LatLonPoint::new(-90., 0.),
            WorldTectonicsIndex::Point(point) => point,
        }
    }
}
