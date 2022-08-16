use crate::components::world::latlon::{LatLonPoint, ValuePoint, LATITUDE_RANGE, LONGITUDE_RANGE};
use std::fmt::Debug;
use utils::iterators::WorldPointsIterator;
use utils::WorldTectonicsIndex;

pub mod height;
pub mod latlon;
pub mod map_modes;
pub mod render;
pub mod resources;
pub mod tectonics;
pub mod utils;

pub const PIXEL_SIZE: f32 = 12.;
pub const PIXEL_BUFFER: f32 = 5.;
pub const TECTONIC_PRECISION: u32 = 2;

#[derive(Debug, Clone)]
pub struct WorldPoints<T>
where
    T: Debug + Clone,
{
    precision: u32,
    north_pole_point: ValuePoint<T>,
    south_pole_point: ValuePoint<T>,
    points: Vec<ValuePoint<T>>,
}

impl<T> WorldPoints<T>
where
    T: Debug + Clone,
{
    /// Create a new world, where each points value is determined by passing the lat/lon through a function
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

        let points: Vec<ValuePoint<T>> = {
            let mut point_vec =
                Vec::with_capacity(WorldPoints::<T>::precision_points_len(precision));

            // i.e. precision of 2 => 89.5 to -89.5
            for lat_index in 1..(LATITUDE_RANGE as u32 * precision) as u32 {
                // @ 1 => 180. - .5 - 90 = 89.5
                // @ 359 (last one) => 180. - 179.5 - 90. = -89.5
                let lat =
                    LATITUDE_RANGE - (lat_index as f32) / precision as f32 - (LATITUDE_RANGE / 2.);

                // i.e. precision of 2 = -179.5 to 180.0
                for lon_index in 1..=(LONGITUDE_RANGE as u32 * precision) as i32 {
                    let lon = (lon_index as f32) / precision as f32 - (LONGITUDE_RANGE / 2.);

                    let lat_lon_point = LatLonPoint::new(lat, lon);
                    let value = point_func(WorldTectonicsIndex::from(lat_lon_point));

                    point_vec.push(ValuePoint::new(lat_lon_point, value));
                }
            }
            point_vec
        };

        Self {
            precision,
            north_pole_point,
            south_pole_point,
            points,
        }
    }

    pub fn iter<'a>(&'a self) -> WorldPointsIterator<'a, T> {
        WorldPointsIterator::new(self)
    }

    pub fn precision_points_len(precision: u32) -> usize {
        (LATITUDE_RANGE as usize * precision as usize - 1)
            * (LONGITUDE_RANGE as usize * precision as usize)
    }

    pub fn points_len(&self) -> usize {
        WorldPoints::<T>::precision_points_len(self.precision)
    }
}
