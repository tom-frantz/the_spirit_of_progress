use crate::latlon::*;
use crate::tectonics::utils::iterators::*;
use crate::tectonics::utils::WorldTectonicsIndex;
use std::collections::HashMap;
use std::fmt::Debug;

pub mod height;
pub mod plates;
pub mod utils;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

#[derive(Debug)]
pub struct WorldPoints<T>
where
    T: Debug + Clone,
{
    precision: u32,
    north_pole_point: ValuePoint<T>,
    south_pole_point: ValuePoint<T>,
    points: HashMap<LatLonPoint, ValuePoint<T>>,
}

impl<T> WorldPoints<T>
where
    T: Debug + Clone,
{
    pub fn new<F>(precision: u32, point_func: F) -> Self
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
            for lat_index in 1..(LATITUDE_RANGE as u32 * precision) as i32 {
                let lat = (lat_index as f32) / precision as f32 - (LATITUDE_RANGE / 2.);

                // i.e. precision of 2 = -179.5 to 180.0
                for lon_index in 1..=(LONGITUDE_RANGE as u32 * precision) as i32 {
                    let lon = (lon_index as f32) / precision as f32 - (LONGITUDE_RANGE / 2.);

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
