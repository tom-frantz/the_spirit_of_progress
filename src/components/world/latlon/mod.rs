use crate::components::world::latlon::utils::wrap_lon;
use crate::components::world::utils::{lon_index_range, lon_index_to_value};
use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use rand::Rng;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Deref, DerefMut};

pub mod utils;

// Horizontal running lines
// Run parallel to each other and the equator.
pub const LATITUDE_RANGE: f32 = 180.;

// Vertical running lines
// All touch both poles, intersect equator.
pub const LONGITUDE_RANGE: f32 = 360.;

pub trait WorldPoint {
    // [-90, 90]
    fn latitude(&self) -> f32;
    // (-180, 180]
    fn longitude(&self) -> f32;

    fn lat(&self) -> f32 {
        self.latitude()
    }
    fn lon(&self) -> f32 {
        self.longitude()
    }

    fn point(&self) -> LatLonPoint {
        LatLonPoint::new(self.lat(), self.lon())
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct LatLonPoint {
    latitude: f32,
    longitude: f32,
}

impl Add<LatLonPoint> for LatLonPoint {
    type Output = LatLonPoint;

    fn add(self, rhs: LatLonPoint) -> Self::Output {
        let mut lat = self.lat() + rhs.lat();
        if lat > 90. {
            lat = 90. - (90. - lat)
        } else if lat < -90. {
            lat = -90. + (-90. - lat)
        }
        let lon = wrap_lon(self.lon() + rhs.lon());

        LatLonPoint::new(lat, lon)
    }
}

impl<'a, 'b> Add<&'a LatLonPoint> for &'b LatLonPoint {
    type Output = LatLonPoint;

    fn add(self, rhs: &'a LatLonPoint) -> Self::Output {
        LatLonPoint::new(self.lat() + rhs.lat(), self.lon() + rhs.lon())
    }
}

impl LatLonPoint {
    pub fn new(mut latitude: f32, longitude: f32) -> Self {
        if latitude > 90. {
            latitude = 90. - (latitude - 90.)
        } else if latitude < -90. {
            latitude = -90. + (latitude + 90.)
        }

        // latitude should be fixed by above. if not, break this.
        assert!(latitude <= 90., "{latitude}");
        assert!(latitude >= -90.);

        LatLonPoint {
            latitude,
            longitude: wrap_lon(longitude),
        }
    }

    pub fn unbounded_new(latitude: f32, longitude: f32) -> Self {
        LatLonPoint {
            latitude,
            longitude,
        }
    }

    pub fn random(precision: u32) -> LatLonPoint {
        let mut rng = rand::thread_rng();
        let lat = (rng.gen_range(0..=LATITUDE_RANGE as u32 * precision) / precision) as f32
            - (LATITUDE_RANGE / 2.);
        let lon = (rng.gen_range(1..=LONGITUDE_RANGE as u32 * precision) / precision) as f32
            - (LONGITUDE_RANGE / 2.);

        LatLonPoint::new(lat, lon)
    }

    /// Distance via the central angle of a great circle segment.
    /// https://en.wikipedia.org/wiki/Great-circle_distance
    /// This isn't optimized for very small distances (f32 floating point errors may occur)
    pub fn distance(&self, other: &LatLonPoint) -> f32 {
        let lat1 = self.lat().to_radians();
        let lon1 = self.lon().to_radians();

        let lat2 = other.lat().to_radians();
        let lon2 = other.lon().to_radians();

        let lat_sin = lat1.sin() * lat2.sin();
        let lat_cos = lat1.cos() * lat2.cos();
        let delta_lon = (lon1 - lon2).abs();

        f32::acos(lat_sin + lat_cos * delta_lon.cos())
    }

    pub fn tile_pos(&self, precision: u32) -> TilePos {
        TilePos {
            y: ((self.lat() + (LATITUDE_RANGE / 2.)) * precision as f32) as u32,
            x: ((self.lon() + (LONGITUDE_RANGE / 2.)) * precision as f32) as u32,
        }
    }

    pub fn neighbours(&self, precision: u32) -> Vec<LatLonPoint> {
        if self.lat() == 90. {
            let mut points = Vec::new();

            for lon in lon_index_range(precision) {
                let lon_value = lon_index_to_value(lon, precision);
                points.push(LatLonPoint::new(90. - 1. / precision as f32, lon_value))
            }

            points
        } else if self.lat() == -90. {
            let mut points = Vec::new();

            for lon in lon_index_range(precision) {
                let lon_value = lon_index_to_value(lon, precision);
                points.push(LatLonPoint::new(-90. + 1. / precision as f32, lon_value))
            }

            points
        } else {
            let mut points: Vec<LatLonPoint> = Vec::new();
            let step = 1. / precision as f32;
            if self.lat() == 90. - step {
                points.push(LatLonPoint::new(90., 0.))
            } else {
                points.push(LatLonPoint::new(
                    self.lat() + step,
                    wrap_lon(self.lon() - step),
                ));
                points.push(LatLonPoint::new(self.lat() + step, self.lon()));
                points.push(LatLonPoint::new(
                    self.lat() + step,
                    wrap_lon(self.lon() + step),
                ));
            }
            points.push(LatLonPoint::new(self.lat(), wrap_lon(self.lon() + step)));

            if self.lat() == -90. + step {
                points.push(LatLonPoint::new(-90., 0.))
            } else {
                points.push(LatLonPoint::new(
                    self.lat() - step,
                    wrap_lon(self.lon() - step),
                ));
                points.push(LatLonPoint::new(self.lat() - step, self.lon()));
                points.push(LatLonPoint::new(
                    self.lat() - step,
                    wrap_lon(self.lon() + step),
                ));
            }
            points.push(LatLonPoint::new(self.lat(), wrap_lon(self.lon() - step)));

            points
        }
    }
}

impl WorldPoint for LatLonPoint {
    fn latitude(&self) -> f32 {
        self.latitude
    }

    fn longitude(&self) -> f32 {
        self.longitude
    }
}

impl From<Vec2> for LatLonPoint {
    fn from(vec2: Vec2) -> Self {
        Self {
            latitude: vec2.x,
            longitude: vec2.y,
        }
    }
}

impl Hash for LatLonPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32((self.latitude() * 2.0) as u32);
        state.write_u32((self.longitude() * 2.0) as u32);
    }
}

impl Eq for LatLonPoint {}

#[derive(Debug, Clone)]
pub struct ValuePoint<T>
where
    T: Debug + Clone,
{
    pub point: LatLonPoint,
    pub value: T,
}

impl<T> ValuePoint<T>
where
    T: Debug + Clone,
{
    pub fn new(point: &LatLonPoint, value: T) -> Self {
        Self {
            point: point.clone(),
            value,
        }
    }
}

impl<T> Deref for ValuePoint<T>
where
    T: Debug + Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ValuePoint<T>
where
    T: Debug + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> WorldPoint for ValuePoint<T>
where
    T: Debug + Clone,
{
    fn latitude(&self) -> f32 {
        self.point.latitude()
    }

    fn longitude(&self) -> f32 {
        self.point.longitude()
    }
}

impl<'a, T> WorldPoint for &'a ValuePoint<T>
where
    T: Debug + Clone,
{
    fn latitude(&self) -> f32 {
        self.point.latitude()
    }

    fn longitude(&self) -> f32 {
        self.point.longitude()
    }
}
