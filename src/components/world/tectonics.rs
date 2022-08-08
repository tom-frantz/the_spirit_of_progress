use crate::latlon::*;
use crate::tectonics::utils::iterators::*;
use crate::tectonics::utils::WorldTectonicsIndex;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod utils;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

#[derive(Debug)]
pub struct WorldPoints<T>
where
    T: Debug + Clone,
{
    precision: f32,
    north_pole_point: ValuePoint<T>,
    south_pole_point: ValuePoint<T>,
    points: HashMap<LatLonPoint, ValuePoint<T>>,
}

impl<T> WorldPoints<T>
where
    T: Debug + Clone,
{
    pub fn new_with_func<F>(precision: f32, point_func: F) -> Self
    where
        F: Fn(WorldTectonicsIndex) -> T,
    {
        let north_pole_point = ValuePoint::new(
            WorldTectonicsIndex::NorthPole.into(),
            point_func(WorldTectonicsIndex::NorthPole),
        );

        let south_pole_point = ValuePoint::new(
            WorldTectonicsIndex::SouthPole.into(),
            point_func(WorldTectonicsIndex::SouthPole),
        );

        let points: HashMap<LatLonPoint, ValuePoint<T>> = {
            let mut point_dict = HashMap::new();

            // i.e. precision of 2 => -89.5 to 89.5
            'latitude_loop: for mut lat_index in 1..(LATITUDE_RANGE * precision) as i32 {
                let lat = (lat_index as f32) / precision - (LATITUDE_RANGE / 2.);

                // i.e. precision of 2 = -179.5 to 180.0
                'longitude_loop: for mut lon_index in 1..=(LONGITUDE_RANGE * precision) as i32 {
                    let lon = (lon_index as f32) / precision - (LONGITUDE_RANGE / 2.);

                    let lat_lon_point = LatLonPoint::new(lat, lon);
                    let value = point_func(WorldTectonicsIndex::from(lat_lon_point));

                    point_dict.insert(lat_lon_point, ValuePoint::new(lat_lon_point, value));
                }
            }

            point_dict
        };

        Self {
            precision,
            north_pole_point,
            south_pole_point,
            points,
        }
    }

    pub fn iter(&self) -> WorldTectonicsIterator<T> {
        WorldTectonicsIterator::new(self)
    }

    pub fn into_iter(self) -> WorldTectonicsIntoIterator<T> {
        WorldTectonicsIntoIterator::new(self)
    }
}

impl WorldPoints<f32> {
    pub fn new(
        precision: f32,
        north_pole: f32,
        south_pole: f32,
        points: HashMap<LatLonPoint, ValuePoint<f32>>,
    ) -> Self {
        assert!(precision.fract() <= f32::EPSILON);

        Self {
            precision,
            north_pole_point: ValuePoint::new(LatLonPoint::new(0.0, 90.0), north_pole),
            south_pole_point: ValuePoint::new(LatLonPoint::new(0.0, -90.0), south_pole),
            points,
        }
    }
}
