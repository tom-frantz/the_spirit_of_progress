use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use rand::Rng;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

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
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct LatLonPoint(Vec2);

impl LatLonPoint {
    pub fn new(lat: f32, lon: f32) -> Self {
        LatLonPoint(Vec2::new(lat, lon))
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
        let delta_lon = (lon1 - lon2);

        f32::acos(lat_sin + lat_cos * delta_lon.cos())
    }

    pub fn tile_pos(self, precision: u32) -> TilePos {
        TilePos {
            y: ((self.lat() + (LATITUDE_RANGE / 2.)) * precision as f32) as u32,
            x: ((self.lon() + (LONGITUDE_RANGE / 2.)) * precision as f32) as u32,
        }
    }
}

impl WorldPoint for LatLonPoint {
    fn latitude(&self) -> f32 {
        self.0.x
    }

    fn longitude(&self) -> f32 {
        self.0.y
    }
}

impl From<Vec2> for LatLonPoint {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
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
    pub fn new(point: LatLonPoint, value: T) -> Self {
        Self { point, value }
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
