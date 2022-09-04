use crate::components::world::latlon::{
    LatLonPoint, ValuePoint, WorldPoint, LATITUDE_RANGE, LONGITUDE_RANGE,
};
use crate::components::world::utils::iterators::MutWorldPointsIterator;
use crate::components::world::utils::{
    lat_index_range, lat_index_to_value, lon_index_range, lon_index_to_value, precision_points_len,
};
use std::fmt::Debug;
use utils::iterators::WorldPointsIterator;
use utils::WorldTectonicsIndex;

pub mod height;
pub mod latlon;
pub mod render;
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
            &WorldTectonicsIndex::NorthPole.into(),
            point_func(WorldTectonicsIndex::NorthPole),
        );

        let south_pole_point = ValuePoint::new(
            &WorldTectonicsIndex::SouthPole.into(),
            point_func(WorldTectonicsIndex::SouthPole),
        );

        let points: Vec<ValuePoint<T>> = {
            let mut point_vec = Vec::with_capacity(precision_points_len(precision));

            // i.e. precision of 2 => 89.5 to -89.5
            for lat_index in lat_index_range(precision) {
                // @ 1 => 180. - .5 - 90 = 89.5
                // @ 359 (last one) => 180. - 179.5 - 90. = -89.5
                let lat = lat_index_to_value(lat_index, precision);

                // i.e. precision of 2 = -179.5 to 180.0
                for lon_index in lon_index_range(precision) {
                    let lon = lon_index_to_value(lon_index, precision);

                    let lat_lon_point = LatLonPoint::new(lat, lon);
                    let value = point_func(WorldTectonicsIndex::from(lat_lon_point));

                    point_vec.push(ValuePoint::new(&lat_lon_point, value));
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

    pub fn new_fill(precision: u32, value: T) -> Self {
        let mut points: Vec<ValuePoint<T>> = Vec::with_capacity(precision_points_len(precision));

        for lat_index in lat_index_range(precision) {
            // @ 1 => 180. - .5 - 90 = 89.5
            // @ 359 (last one) => 180. - 179.5 - 90. = -89.5
            let lat = lat_index_to_value(lat_index, precision);

            // i.e. precision of 2 = -179.5 to 180.0
            for lon_index in lon_index_range(precision) {
                let lon = lon_index_to_value(lon_index, precision);

                let lat_lon_point = LatLonPoint::new(lat, lon);
                let value = value.clone();

                points.push(ValuePoint::new(&lat_lon_point, value));
            }
        }

        let north_pole_point =
            ValuePoint::new(&WorldTectonicsIndex::NorthPole.into(), value.clone());

        let south_pole_point = ValuePoint::new(&WorldTectonicsIndex::SouthPole.into(), value);

        Self {
            precision,
            north_pole_point,
            south_pole_point,
            points,
        }
    }

    pub fn iter(&self) -> WorldPointsIterator<T> {
        WorldPointsIterator::new(self)
    }

    pub fn iter_mut(&mut self) -> MutWorldPointsIterator<T> {
        MutWorldPointsIterator::new(self)
    }

    pub fn get(&self, point: &LatLonPoint) -> &ValuePoint<T> {
        let index = WorldTectonicsIndex::from(point);
        match &index {
            WorldTectonicsIndex::NorthPole => &self.south_pole_point,
            WorldTectonicsIndex::SouthPole => &self.south_pole_point,
            WorldTectonicsIndex::Point(_) => &self.points[index.vec_index(self.precision)],
        }
    }

    pub fn get_mut(&mut self, point: &LatLonPoint) -> &mut ValuePoint<T> {
        let index = WorldTectonicsIndex::from(point);
        match &index {
            WorldTectonicsIndex::NorthPole => &mut self.south_pole_point,
            WorldTectonicsIndex::SouthPole => &mut self.south_pole_point,
            WorldTectonicsIndex::Point(_) => &mut self.points[index.vec_index(self.precision)],
        }
    }

    pub fn set(&mut self, point: &LatLonPoint, value: T) {
        let index = WorldTectonicsIndex::from(point);

        match &index {
            WorldTectonicsIndex::NorthPole => self.north_pole_point.value = value,
            WorldTectonicsIndex::SouthPole => self.south_pole_point.value = value,
            WorldTectonicsIndex::Point(_) => {
                let vec_index = index.vec_index(self.precision);
                self.points.remove(vec_index);
                self.points.insert(vec_index, ValuePoint::new(point, value));
            }
        }
    }

    pub fn points_len(&self) -> usize {
        precision_points_len(self.precision)
    }
}

#[cfg(test)]
mod test {
    use crate::components::world::latlon::{LatLonPoint, WorldPoint};
    use crate::components::world::utils::WorldTectonicsIndex;
    use crate::components::world::WorldPoints;

    #[test]
    fn testing() {
        let world = WorldPoints::new(2, |point| match point {
            WorldTectonicsIndex::Point(p) => {
                if p.lat() == 82.5 && p.lon() == 11.5 {
                    10.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        });

        let test_point = world.get(&LatLonPoint::new(82.5, 11.5));
        assert_eq!(test_point.value, 10.);
    }
}
