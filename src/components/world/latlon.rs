use crate::tectonics::WorldTectonics;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::render_resource::std430::Std430;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub mod utils;

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct LatLonPoint(Vec2);

impl LatLonPoint {
    pub fn validate(&self) {
        // assert!(self.latitude() >= -180.0);
        // assert!(self.latitude() <= 180.0);
        // assert!(self.longitude() >= -90.0);
        // assert!(self.longitude() <= 90.0);
    }

    pub fn new(lat: f32, lon: f32) -> Self {
        let value = LatLonPoint(Vec2::new(lat, lon));
        value.validate();
        value
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

#[derive(Debug)]
pub struct ValuePoint<T>
where
    T: Debug,
{
    pub point: LatLonPoint,
    pub value: T,
}

impl<T> ValuePoint<T>
where
    T: Debug,
{
    pub fn new(point: LatLonPoint, value: T) -> Self {
        Self { point, value }
    }
}

impl<T> WorldPoint for ValuePoint<T>
where
    T: Debug,
{
    fn latitude(&self) -> f32 {
        self.point.latitude()
    }

    fn longitude(&self) -> f32 {
        self.point.longitude()
    }
}
