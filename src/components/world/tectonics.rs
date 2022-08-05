use crate::latlon::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::thread::current;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

#[derive(Debug)]
pub struct WorldTectonics<T>
where
    T: Debug,
{
    precision: f32,
    north_pole_point: ValuePoint<T>,
    south_pole_point: ValuePoint<T>,
    points: HashMap<LatLonPoint, ValuePoint<T>>,
}

#[derive(Debug)]
pub enum WorldTectonicIndex<T>
where
    T: Debug,
{
    NorthPole,
    SouthPole,
    LatLong(T),
}

#[derive(Debug)]
pub struct WorldTectonicsIterator<'a, T>
where
    T: Debug,
{
    world: &'a WorldTectonics<T>,
    current: Option<LatLonPoint>,
}

impl<'a, T> WorldTectonicsIterator<'a, T>
where
    T: Debug,
{
    fn new(world: &'a WorldTectonics<T>) -> Self {
        Self {
            world,
            current: Some(LatLonPoint::new(90.0, 0.0)),
        }
    }
}

impl<'a, T> Iterator for WorldTectonicsIterator<'a, T>
where
    T: Debug,
{
    type Item = &'a ValuePoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(point) = self.current {
            if point == LatLonPoint::new(0.0, 90.0) {
                self.current = Some(LatLonPoint::new(-179.5, 89.5));
                Some(&self.world.north_pole_point)
            } else if point == LatLonPoint::new(0.0, -90.0) {
                self.current = None;
                Some(&self.world.south_pole_point)
            } else {
                let mut lat = point.lat() + 0.5;
                let mut lon = point.lon();

                if lat == 180.0 {
                    lat = -179.5;
                    lon -= 0.5;
                    if lon == -90.0 {
                        lat = 0.0;
                    }
                }

                self.current = Some(LatLonPoint::new(lat, lon));
                Some(&self.world.points[&point])
            }
        } else {
            None
        };
    }
}

impl WorldTectonics<f32> {
    pub fn new(
        precision: f32,
        north_pole: f32,
        south_pole: f32,
        points: HashMap<LatLonPoint, ValuePoint<f32>>,
    ) -> Self {
        assert!((1.0 / precision).fract() <= f32::EPSILON);

        Self {
            precision,
            north_pole_point: ValuePoint::new(LatLonPoint::new(0.0, 90.0), north_pole),
            south_pole_point: ValuePoint::new(LatLonPoint::new(0.0, -90.0), south_pole),
            points,
        }
    }

    pub fn iter(&self) -> WorldTectonicsIterator<f32> {
        WorldTectonicsIterator {
            world: self,
            current: Some(LatLonPoint::new(0.0, 90.0)),
        }
    }

    // fn point_index(lat: f32, lon: f32) -> WorldTectonicIndex<usize> {
    //     // Bottom up, -180 -> 180
    //     let lat_index: f32;
    //     if lon == 90.0 {
    //         return WorldTectonicIndex::NorthPole
    //     } else if lon == -90.0 {
    //         return WorldTectonicIndex::SouthPole
    //     } else {
    //         lat_index = (if lat == -180.0 { 180.0 } else { lat } + 180.0) / DEGREE_STEP_INTERVAL
    //     }
    //
    //     let lon_index = (lon + 90.0) / DEGREE_STEP_INTERVAL;
    //
    //     return WorldTectonicIndex::LatLong(0);
    // }
}

impl<T> WorldTectonics<T>
where
    T: Debug,
{
    // pub fn point(&self, lat: f32, lon: f32) -> &T {
    //     let index = WorldTectonics::point_index(lat, lon);
    //     match index {
    //         WorldTectonicIndex::NorthPole => &self.north_pole_point.value,
    //         WorldTectonicIndex::SouthPole => &self.south_pole_point.value,
    //         WorldTectonicIndex::LatLong(point) => &self.points[point].value
    //     }
    // }
}

// #[cfg(test)]
// mod test {
//     use crate::{UVec2, WorldTectonics};
//     use crate::tectonics::WorldTectonicIndex;
//
//     #[test]
//     fn indexes_correctly() {
//         let world: WorldTectonics<f32> = WorldTectonics::new(0.5, 0.0, 0.0, vec![]);
//
//         for lon_range in -180..=180 {
//             // -90, 90
//             let lon = lon_range as f32 / 2.0;
//
//             for lat_range in -360..=360 {
//                 // -180, 180
//                 let lat = lat_range as f32 / 2.0;
//
//                 let actual = WorldTectonics::point_index(lat, lon);
//
//                 if lon == -90.0 {
//                     assert_eq!(actual, WorldTectonicIndex::SouthPole)
//                 } else if lon == 90.0 {
//                     assert_eq!(actual, WorldTectonicIndex::NorthPole)
//                 } else {
//                     let lat_expected = if lat == -180.0 {
//                         720
//                     } else {
//                         (lat_range + 360) as u32
//                     };
//                     let lon_expected = (lon_range + 180) as u32;
//                     // println!(
//                     //     "Current: {} {}, or {}, {}. Expected: {} {}",
//                     //     lat, lon, lat_range, lon_range, lat_expected, lon_expected
//                     // );
//                     assert_eq!(actual, UVec2::new(lat_expected, lon_expected))
//                 }
//             }
//         }
//     }
// }
