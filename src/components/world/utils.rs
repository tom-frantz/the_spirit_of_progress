use crate::components::world::latlon::{
    LatLonPoint, ValuePoint, WorldPoint, LATITUDE_RANGE, LONGITUDE_RANGE,
};
use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};

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

pub fn lat_index_range(precision: u32) -> Range<u32> {
    1..(LATITUDE_RANGE as u32 * precision) as u32
}

pub fn lon_index_range(precision: u32) -> RangeInclusive<u32> {
    1..=(LONGITUDE_RANGE as u32 * precision) as u32
}

pub fn lat_index_to_value(index: u32, precision: u32) -> f32 {
    LATITUDE_RANGE - (index as f32) / precision as f32 - (LATITUDE_RANGE / 2.)
}

pub fn lon_index_to_value(index: u32, precision: u32) -> f32 {
    (index as f32) / precision as f32 - (LONGITUDE_RANGE / 2.)
}
