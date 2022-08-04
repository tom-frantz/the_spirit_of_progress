use bevy::prelude::*;
use crate::latlon::*;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

pub struct WorldTectonics<T>
{
    precision: f32,
    north_pole_point: ValuePoint<T>,
    south_pole_point: ValuePoint<T>,
    points: Vec<ValuePoint<T>>,
}

pub enum WorldTectonicIndex<T> {
    NorthPole,
    SouthPole,
    LatLong(T)
}

impl WorldTectonics<f32> {
    pub fn new(precision: f32, north_pole: f32, south_pole: f32, points: Vec<ValuePoint<f32>>) -> Self {
        assert!((1.0 / precision).fract() <= f32::EPSILON);

        Self {
            precision,
            north_pole_point: ValuePoint::new(Vec2::new(90.0, 0.0),north_pole),
            south_pole_point: ValuePoint::new(Vec2::new(-90.0, 0.0),south_pole),
            points
        }
    }

    fn point_index(lat: f32, lon: f32) -> WorldTectonicIndex<usize> {
        // Bottom up, -180 -> 180
        let lat_index: f32;
        if lon == 90.0 {
            return WorldTectonicIndex::NorthPole
        } else if lon == -90.0 {
            return WorldTectonicIndex::SouthPole
        } else {
            lat_index = (if lat == -180.0 { 180.0 } else { lat } + 180.0) / DEGREE_STEP_INTERVAL
        }

        let lon_index = (lon + 90.0) / DEGREE_STEP_INTERVAL;

        return WorldTectonicIndex::LatLong(0);
    }
}

impl<T> WorldTectonics<T>
where
    T: WorldPoint,
{
    pub fn point(&self, lat: f32, lon: f32) -> &T {
        let index = WorldTectonics::point_index(lat, lon);
        match index {
            WorldTectonicIndex::NorthPole => &self.north_pole_point.value,
            WorldTectonicIndex::SouthPole => &self.south_pole_point.value,
            WorldTectonicIndex::LatLong(point) => &self.points[point].value
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{UVec2, WorldTectonics};
    use crate::tectonics::WorldTectonicIndex;

    #[test]
    fn indexes_correctly() {
        let world: WorldTectonics<f32> = WorldTectonics::new(0.5, 0.0, 0.0, vec![]);

        for lon_range in -180..=180 {
            // -90, 90
            let lon = lon_range as f32 / 2.0;

            for lat_range in -360..=360 {
                // -180, 180
                let lat = lat_range as f32 / 2.0;

                let actual = WorldTectonics::point_index(lat, lon);

                if lon == -90.0 {
                    assert_eq!(actual, WorldTectonicIndex::SouthPole)
                } else if lon == 90.0 {
                    assert_eq!(actual, WorldTectonicIndex::NorthPole)
                } else {
                    let lat_expected = if lat == -180.0 {
                        720
                    } else {
                        (lat_range + 360) as u32
                    };
                    let lon_expected = (lon_range + 180) as u32;
                    // println!(
                    //     "Current: {} {}, or {}, {}. Expected: {} {}",
                    //     lat, lon, lat_range, lon_range, lat_expected, lon_expected
                    // );
                    assert_eq!(actual, UVec2::new(lat_expected, lon_expected))
                }
            }
        }
    }
}
