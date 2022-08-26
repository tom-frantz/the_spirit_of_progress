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

impl WorldTectonicsIndex {
    pub fn vec_index(&self, precision: u32) -> usize {
        match self {
            WorldTectonicsIndex::NorthPole => panic!("a"),
            WorldTectonicsIndex::SouthPole => panic!("b"),
            WorldTectonicsIndex::Point(point) => {
                assert_ne!(point.lat(), 90.);
                assert_ne!(point.lat(), -90.);

                let f32_precision = (precision as f32);
                let step = 1. / f32_precision;

                let lat_index = ((point.lat() - 90. + step).abs() * f32_precision) as usize
                    * (LONGITUDE_RANGE as usize * precision as usize - 1);
                let lon_index = (point.lon() + 180.) * f32_precision - 1.;

                return lat_index + lon_index as usize;
            }
        }
    }
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
    // excludes both extremes (-90, 90)
    1..(LATITUDE_RANGE as u32 * precision) as u32
}

pub fn lon_index_range(precision: u32) -> RangeInclusive<u32> {
    // excludes bottom extremes (-180)
    1..=(LONGITUDE_RANGE as u32 * precision) as u32
}

pub fn lat_index_to_value(index: u32, precision: u32) -> f32 {
    LATITUDE_RANGE - (index as f32) / precision as f32 - (LATITUDE_RANGE / 2.)
}

pub fn lon_index_to_value(index: u32, precision: u32) -> f32 {
    (index as f32) / precision as f32 - (LONGITUDE_RANGE / 2.)
}

// pub fn lat_value_to_index(value: f32, precision: u32) -> u32 {
//
// }
//
// pub fn lon_value_to_index(value: f32, precision: u32) -> u32 {
//
// }

pub fn precision_points_len(precision: u32) -> usize {
    (LATITUDE_RANGE as usize * precision as usize - 1)
        * (LONGITUDE_RANGE as usize * precision as usize)
}
