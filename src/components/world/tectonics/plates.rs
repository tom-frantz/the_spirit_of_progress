use crate::components::world::latlon::{LatLonPoint, ValuePoint};
use crate::components::world::render::{TileRender, WorldRender};
use crate::components::world::tectonics::WorldPoints;
use crate::ui::theme::{Agriculture, Colour, MenuColour};
use bevy::prelude::Color;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::*;

#[derive(Debug, Clone)]
pub struct Plate {
    id: u32,
    origin: LatLonPoint,
    // How slowly the strength will decay.
    spread: f32,
    // How likely this plate will be the dominant one.
    strength: f32,
    pub colour: Agriculture,
}

#[derive(Debug, Clone)]
pub struct PlatePoint {
    plate_id: u32,
    strength: f32,
}

impl TileRender for PlatePoint {
    type World = TectonicPlates;

    fn bundle(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileBundle {
        let mut color = world.plates[&self.plate_id].colour.tile_color();

        for (_id, plate) in &world.plates {
            let origin_pos = plate.origin.tile_pos(world.precision());
            if origin_pos.x == position.x && origin_pos.y == position.y {
                color = MenuColour::BlackPen.tile_color()
            }
        }

        TileBundle {
            position,
            tilemap_id,
            color,
            ..Default::default()
        }
    }
}

impl PlatePoint {
    pub fn new(plate_id: u32, strength: f32) -> Self {
        PlatePoint { plate_id, strength }
    }
}

#[derive(Debug)]
pub struct TectonicPlates {
    pub world: WorldPoints<PlatePoint>,
    plates: HashMap<u32, Plate>,
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

        for id in 0..major_plates {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = rand::random();

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    spread: 50.0,
                    strength: 10.0,
                    colour,
                },
            );
        }

        for id in major_plates..(major_plates + minor_plates) {
            let point: LatLonPoint = LatLonPoint::random(precision);
            let colour = rand::random();

            plates.insert(
                id,
                Plate {
                    id,
                    origin: point,
                    spread: 10.0,
                    strength: 20.0,
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

impl WorldRender for TectonicPlates {
    fn precision(&self) -> u32 {
        self.world.precision
    }
}
