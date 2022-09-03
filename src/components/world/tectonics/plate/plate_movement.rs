use crate::components::world::LatLonPoint;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlateMovement {
    vertical: i32,
    horizontal: i32,
}

impl PlateMovement {
    pub fn new(vertical: i32, horizontal: i32) -> Self {
        PlateMovement {
            vertical,
            horizontal,
        }
    }

    pub fn to_point(&self, precision: u32) -> LatLonPoint {
        let lat_lon_step = 1. / (precision as f32) as f32;
        LatLonPoint::new(
            lat_lon_step * self.vertical as f32,
            lat_lon_step * self.horizontal as f32,
        )
    }

    pub fn vertical(&self) -> i32 {
        self.vertical
    }
    pub fn horizontal(&self) -> i32 {
        self.horizontal
    }
}

impl Add for PlateMovement {
    type Output = PlateMovement;

    fn add(self, rhs: Self) -> Self::Output {
        PlateMovement {
            vertical: self.vertical + rhs.vertical,
            horizontal: self.horizontal + rhs.horizontal,
        }
    }
}

impl Sub for PlateMovement {
    type Output = PlateMovement;

    fn sub(self, rhs: Self) -> Self::Output {
        PlateMovement {
            vertical: self.vertical - rhs.vertical,
            horizontal: self.horizontal - rhs.horizontal,
        }
    }
}
