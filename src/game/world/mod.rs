pub use self::collections::{CellData, HexWorldCell, HexWorldData};

use self::elevation::WorldElevationData;
use self::tectonics::WorldTectonicsData;
use bevy::prelude::*;

mod collections;

pub mod elevation;
pub mod tectonics;

#[derive(Component, Debug, Default)]
pub struct HexWorld;

#[derive(Bundle)]
pub struct HexWorldBundle {
    world: HexWorld,
    elevations: WorldElevationData,
    tectonics: WorldTectonicsData,
}

impl HexWorld {
    pub fn new() -> HexWorldBundle {
        println!("HERE????????");
        let elevations = WorldElevationData::default();
        let tectonics = WorldTectonicsData::default();

        HexWorldBundle {
            world: HexWorld,
            elevations,
            tectonics,
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
