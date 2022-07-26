use bevy::prelude::*;

pub trait WorldPoint {
    // Vertical running lines: (-180, 180]
    fn latitude(&self) -> f32;
    // Horizontal running lines: [-90, 90]
    fn longitude(&self) -> f32;

    fn lat(&self) -> f32 {
        self.latitude()
    }
    fn lon(&self) -> f32 {
        self.longitude()
    }
}

impl WorldPoint for Vec2 {
    fn latitude(&self) -> f32 {
        self.x
    }
    fn longitude(&self) -> f32 {
        self.y
    }
}

impl WorldPoint for UVec2 {
    fn latitude(&self) -> f32 {
        self.x as f32
    }
    fn longitude(&self) -> f32 {
        self.y as f32
    }
}

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

fn wrap_latitude(latitude: f32) -> f32 {
    let mut lat = latitude;
    while lat <= -180.0 {
        lat += 360.0;
    }
    lat
}

#[derive(Default)]
pub struct WorldTectonics<T>
where
    T: WorldPoint,
{
    points: Vec<Vec<T>>,
}

impl WorldTectonics<UVec2> {
    fn point_index(lat: f32, lon: f32) -> UVec2 {
        // Bottom up, -180 -> 180
        let lat_index: f32;
        if lon == 90.0 || lon == -90.0 {
            lat_index = 0.0
        } else {
            lat_index = (if lat == -180.0 { 180.0 } else { lat } + 180.0) / DEGREE_STEP_INTERVAL
        }

        let lon_index = (lon + 90.0) / DEGREE_STEP_INTERVAL;

        return UVec2::new(lat_index as u32, lon_index as u32);
    }
}

impl<T> WorldTectonics<T>
where
    T: WorldPoint,
{
    fn new() -> Self {
        return Self { points: Vec::new() };
    }

    pub fn point(&self, lat: f32, lon: f32) -> &T {
        let index = WorldTectonics::point_index(lat, lon);
        return &self.points[index.x as usize][index.y as usize];
    }
}

#[cfg(test)]
mod test {
    use crate::components::world::tectonics::WorldTectonics;
    use crate::UVec2;

    #[test]
    fn indexes_correctly() {
        let world: WorldTectonics<UVec2> = WorldTectonics::new();
        for lon_range in -180..=180 {
            // -90, 90
            let lon = lon_range as f32 / 2.0;

            for lat_range in -360..=360 {
                // -180, 180
                let lat = lat_range as f32 / 2.0;

                let actual: UVec2 = WorldTectonics::point_index(lat, lon);

                if lon == -90.0 {
                    assert_eq!(actual, UVec2::new(0, 0))
                } else if lon == 90.0 {
                    assert_eq!(actual, UVec2::new(0, 360))
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
