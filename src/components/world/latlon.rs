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
    pub fn distance(&self, other: &LatLonPoint) -> f32 {
        // let sins = self.lat().sin() * other.lat().sin();
        // let coss = self.lat().cos() * other.lat().cos() * (self.lon() - other.lon()).cos();
        // f32::acos(sins + coss)

        ((self.lat() - other.lat()).powf(2.) + (self.lon() - other.lon()).powf(2.)).sqrt()

        // let d_lat: f64 = (other.lat() - self.lat()).abs() as f64;
        // let d_lon: f64 = (other.lon() - self.lon()).abs() as f64;
        // let lat_1: f64 = self.lat().to_radians() as f64;
        // let lat_2: f64 = self.lat().to_radians() as f64;
        //
        // let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        //     + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat_1.cos()) * (lat_2.cos());
        // let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));
        //
        // return c as f32;
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
