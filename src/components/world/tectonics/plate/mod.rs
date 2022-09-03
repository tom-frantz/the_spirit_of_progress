use crate::components::world::latlon::LatLonPoint;
use bevy::prelude::Color;
use plate_movement::PlateMovement;

pub mod plate_boundary;
pub mod plate_movement;

#[derive(Debug, Clone)]
pub struct Plate {
    pub id: u32,
    pub origin: LatLonPoint,
    pub plate_type: PlateType,
    pub age: u32,
    pub size: f32,
    pub colour: Color,

    pub horizontal_speed_inverse: i32,
    pub vertical_speed_inverse: i32,

    pub horizontal_ticks: u32,
    pub vertical_ticks: u32,
}

impl Plate {
    pub fn tick(&mut self) -> PlateMovement {
        self.age += 1;
        self.horizontal_ticks += 1;
        self.vertical_ticks += 1;

        let vertical_movement = if self.vertical_ticks == self.vertical_speed_inverse.abs() as u32 {
            self.vertical_ticks = 0;
            1 * self.vertical_speed_inverse.signum()
        } else {
            0
        };

        let horizontal_movement =
            if self.horizontal_ticks == self.horizontal_speed_inverse.abs() as u32 {
                self.horizontal_ticks = 0;
                1 * self.horizontal_speed_inverse.signum()
            } else {
                0
            };

        PlateMovement::new(vertical_movement, horizontal_movement)
    }

    pub fn plate_drift_speed(&self) -> PlateMovement {
        PlateMovement::new(
            (1. as f32 / self.vertical_speed_inverse as f32) as i32,
            (1. as f32 / self.horizontal_speed_inverse as f32) as i32,
        )
    }
}

#[derive(Clone, Debug)]
pub enum PlateType {
    Oceanic,
    Continental,
}
