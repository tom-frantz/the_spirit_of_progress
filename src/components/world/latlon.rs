use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use crate::tectonics::WorldTectonics;

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

impl WorldPoint for Vec2 {
    fn latitude(&self) -> f32 {
        self.x
    }
    fn longitude(&self) -> f32 {
        self.y
    }
}

pub struct ValuePoint<T> {
    pub point: Vec2,
    pub value: T
}

impl<T> ValuePoint<T> {
    pub fn new(point: Vec2, value: T) -> Self {
        Self {
            point,
            value
        }
    }
}

impl<T> WorldPoint for ValuePoint<T> {
    fn latitude(&self) -> f32 {
        self.point.latitude()
    }

    fn longitude(&self) -> f32 {
        self.point.longitude()
    }
}
