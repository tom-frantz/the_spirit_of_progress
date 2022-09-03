use crate::components::world::height::HeightMap;
use crate::components::world::latlon::{LatLonPoint, ValuePoint, WorldPoint};
use crate::components::world::render::{World, WorldMap};
use crate::components::world::tectonics::plate::plate_boundary::{
    PlateBoundary, PlateBoundaryType,
};
use crate::components::world::tectonics::plate::PlateType;
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::components::world::WorldPoints;
use crate::ui::theme::{Colour, IndustryColour, IndustryColour2, MenuColour};
use bevy::prelude::Color;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use plate::Plate;
use plate::PlateType::*;
use point::PlatePoint;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::cmp::min;
use std::collections::HashSet;
use std::ptr::hash;

pub mod plate;
pub mod point;
pub mod render_modes;

pub const DEGREE_STEP_INTERVAL: f32 = 0.5;

#[derive(Debug, Clone)]
pub struct TectonicsMap {
    pub world: WorldPoints<PlatePoint>,
    pub plates: HashMap<u32, Plate>,
}

impl TectonicsMap {
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

            PlatePoint::new(min_distance.unwrap().0, 0)
        });

        TectonicsMap { world, plates }
    }

    fn generate_plates(
        precision: u32,
        major_plates: u32,
        minor_plates: u32,
    ) -> HashMap<u32, Plate> {
        let mut rng = rand::thread_rng();
        let speed_rng: Uniform<i32> = Uniform::from(1..7);

        let mut sample_speed = move || speed_rng.sample(&mut rng);

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
                    age: id,
                    origin: point,
                    plate_type: Oceanic,
                    size: 1.,
                    colour,
                    vertical_speed_inverse: sample_speed(),
                    horizontal_speed_inverse: sample_speed(),

                    vertical_ticks: 0,
                    horizontal_ticks: 0,
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
                    age: id,
                    origin: point,
                    size: 1.,
                    plate_type: chance,
                    colour,
                    vertical_speed_inverse: sample_speed(),
                    horizontal_speed_inverse: sample_speed(),

                    vertical_ticks: 0,
                    horizontal_ticks: 0,
                },
            );
        }
        plates
    }

    pub fn step(&self, heights: &HeightMap) -> (TectonicsMap, HeightMap) {
        let lat_lon_step = 1. / self.world.precision as f32;

        let mut plate_movement_deltas: HashMap<u32, LatLonPoint> = HashMap::new();

        // Setup the return values; These are the 'next' world states, and are the mutated ones.
        let mut next_tectonics_map: TectonicsMap = self.clone();
        let mut next_height_map: HeightMap = heights.clone();

        // Compute the movement for each plate
        for (_, plate) in &mut next_tectonics_map.plates {
            let movement = plate.tick();
            plate_movement_deltas.insert(plate.id, movement.to_point(self.precision()));
        }

        for plate_point in next_tectonics_map.world.iter_mut() {
            plate_point.tick();
        }

        let point_iter = (&self.world)
            .into_iter()
            .map(|p| (&p.point, &p.value))
            .zip((&heights.world).into_iter().map(|p| &p.value));

        // Iterate over every point in the current world; update the 'next' world accordingly
        for ((src_point, plate_point), height_point) in point_iter {
            let movement_delta = plate_movement_deltas.get(&plate_point.plate_id).unwrap();

            if movement_delta.lat() == 0. && movement_delta.lon() == 0. {
                continue;
            }

            let dest_lat_lon = movement_delta + src_point;
            let dest_point = self.world.get(&dest_lat_lon);

            // Check what type of collision occurs after movement.
            if dest_point.value.plate_id == plate_point.plate_id {
                // Just add in the new one, subtract the current; not going to matter
                let current_height = height_point.height;
                next_height_map.world.get_mut(&src_point).value.height -= current_height;
                next_height_map.world.get_mut(&dest_lat_lon).value.height += current_height;
            } else {
                // The 'next point' is not on the same plate. Handle collision
                let src_plate = &self.plates[&plate_point.plate_id];
                let dest_plate = &self.plates[&dest_point.plate_id];

                let boundary = PlateBoundary::new(src_plate, dest_plate);

                match boundary.boundary_type() {
                    PlateBoundaryType::Convergent(_relative_speed) => match src_plate.plate_type {
                        Oceanic => match dest_plate.plate_type {
                            Oceanic => {
                                // Check which subducts, hence strategy.
                                if src_plate.age >= dest_plate.age {
                                    // src subducts. Reset
                                } else {
                                    // dest subducts. Takeover
                                }
                            }
                            Continental => {
                                // src (oceanic) subducts. Reset.
                            }
                        },
                        Continental => match dest_plate.plate_type {
                            Oceanic => {
                                // dest (oceanic) subducts. Takeover.
                            }
                            Continental => {
                                // don't move, just collision. Reset.
                            }
                        },
                    },
                    PlateBoundaryType::Divergent(_relative_speed) => match src_plate.plate_type {
                        Oceanic => match dest_plate.plate_type {
                            Oceanic => {}
                            Continental => {}
                        },
                        Continental => match dest_plate.plate_type {
                            Oceanic => {}
                            Continental => {}
                        },
                    },
                    PlateBoundaryType::Transform => {}
                }
            }
        }

        (next_tectonics_map, next_height_map)
    }
}

impl<'a> World<'a> for TectonicsMap {
    type Point = PlatePoint;

    fn get_world(&self) -> &WorldPoints<Self::Point> {
        &self.world
    }
}
