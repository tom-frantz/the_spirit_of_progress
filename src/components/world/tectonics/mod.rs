use crate::components::world::latlon::{LatLonPoint, ValuePoint};
use crate::components::world::render::{World, WorldMap};
use crate::components::world::tectonics::PlateType::*;
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::components::world::WorldPoints;
use crate::ui::theme::{Colour, IndustryColour, IndustryColour2, MenuColour};
use bevy::prelude::Color;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use point::PlatePoint;
use std::cmp::min;
use std::collections::HashSet;

pub mod point;
pub mod render_modes;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

#[derive(Clone, Debug)]
pub enum PlateType {
    Oceanic,
    Continental,
}

#[derive(Debug, Clone)]
pub struct Plate {
    id: u32,
    origin: LatLonPoint,
    plate_type: PlateType,
    size: f32,
    pub colour: Color,
}

#[derive(Debug, Clone)]
pub struct TectonicPlates {
    pub world: WorldPoints<PlatePoint>,
    pub plates: HashMap<u32, Plate>,
}

impl TectonicPlates {
    /// Create a full set of new plates - to completion.
    ///
    /// General process on how to do such things
    /// - Seed a bunch of random points
    ///   (True random is fine, it's ok to have points next to each other I guess
    /// - 'Grow' them based on some sort of parameter
    /// - When generating the world, assign them to some points
    /// - Make the currents based on this. Figure out what plates match and make most of the currents go in that direction
    ///
    pub fn new(precision: u32, major_plates: u32, minor_plates: u32) -> Self {
        let mut plates = Self::generate_plates(precision, major_plates, minor_plates);

        let world = WorldPoints::new(precision, |point| {
            let lat_lon: LatLonPoint = point.into();

            let mut min_distance: Option<(u32, f32)> = None;
            for (id, plate) in plates.iter() {
                let weighted_distance = 1. / plate.size * lat_lon.distance(&plate.origin);

                if let Some((_current_id, current_min_distance)) = min_distance {
                    if current_min_distance > weighted_distance {
                        min_distance = Some((id.clone(), weighted_distance))
                    }
                } else {
                    min_distance = Some((id.clone(), weighted_distance));
                }
            }

            PlatePoint::new(min_distance.unwrap().0, 0.)
        });

        TectonicPlates { world, plates }
    }

    fn generate_plates(
        precision: u32,
        major_plates: u32,
        minor_plates: u32,
    ) -> HashMap<u32, Plate> {
        let mut plates = HashMap::new();

        let mut colours = IndustryColour2::vec();

        for id in 0..major_plates {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = colours.remove(0).color();
            if colours.len() == 0 {
                colours = IndustryColour2::vec();
            }

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    plate_type: Oceanic,
                    size: 1.,
                    colour,
                },
            );
        }

        for id in major_plates..(major_plates + minor_plates) {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = colours.remove(0).color();
            if colours.len() == 0 {
                colours = IndustryColour2::vec();
            }

            let chance = if rand::random() { Continental } else { Oceanic };

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    size: 1.,
                    plate_type: chance,
                    colour,
                },
            );
        }
        plates
    }
}

impl<'a> World<'a> for TectonicPlates {
    type Point = PlatePoint;

    fn get_world(&self) -> &WorldPoints<Self::Point> {
        &self.world
    }
}
