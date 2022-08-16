use crate::components::world::latlon::{LatLonPoint, ValuePoint};
use crate::components::world::render::{RenderTheWorld, WorldMap};
use crate::components::world::tectonics::PlateType::*;
use crate::components::world::utils::iterators::{WorldPointsIntoIterator, WorldPointsIterator};
use crate::components::world::WorldPoints;
use crate::ui::theme::{Agriculture, Colour, IndustryColour, MenuColour};
use bevy::prelude::Color;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;
use point::PlatePoint;

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
        let mut plates = HashMap::new();

        let mut colours = IndustryColour::vec();

        for id in 0..major_plates {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = colours.remove(0).color();
            if colours.len() == 0 {
                colours = IndustryColour::vec();
            }

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    plate_type: Oceanic,
                    colour,
                },
            );
        }

        for id in major_plates..(major_plates + minor_plates) {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = colours.remove(0).color();
            if colours.len() == 0 {
                colours = IndustryColour::vec();
            }

            let chance = if rand::random() { Continental } else { Oceanic };

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    plate_type: Continental,
                    colour,
                },
            );
        }

        let world = WorldPoints::new(precision, |point| {
            let lat_lon: LatLonPoint = point.into();

            let mut min_distance: Option<(u32, f32)> = None;
            for (id, plate) in plates.iter() {
                let distance = lat_lon.distance(&plate.origin);

                if let Some((_current_id, current_min_distance)) = min_distance {
                    if current_min_distance > distance {
                        min_distance = Some((id.clone(), distance))
                    }
                } else {
                    min_distance = Some((id.clone(), distance));
                }
            }

            PlatePoint::new(min_distance.unwrap().0, 0.)
        });

        TectonicPlates { world, plates }
    }
}

impl<'a> WorldMap<'a> for TectonicPlates {
    type PointsIterator = WorldPointsIterator<'a, PlatePoint>;
    type Point = &'a ValuePoint<PlatePoint>;

    fn iter_points(&'a self) -> Self::PointsIterator {
        self.world.iter()
    }

    fn precision(&self) -> u32 {
        self.world.precision
    }
}

impl<'a> RenderTheWorld<'a> for PlatePoint {
    type World = TectonicPlates;

    fn get_tile_bundle(
        point: &<Self::World as WorldMap>::Point,
        world: &Self::World,
        tilemap_id: TilemapId,
    ) -> TileBundle {
        TileBundle {
            tilemap_id,
            position: point.point.tile_pos(world.precision()),
            color: TileColor(world.plates[&point.value.plate_id].colour),
            ..Default::default()
        }
    }
}
