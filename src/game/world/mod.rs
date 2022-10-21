pub use self::collections::{CellData, HexWorldCell, HexWorldData};

use self::elevation::WorldElevationData;
use self::tectonics::WorldTectonicsData;
use bevy::prelude::*;

mod collections;

pub mod elevation;
pub mod tectonics;

#[derive(Component, Debug, Copy, Clone)]
pub struct HexWorld {
    size: usize,
}

impl Default for HexWorld {
    fn default() -> Self {
        Self { size: 1600 }
    }
}

impl HexWorld {
    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub enum HexWorldMapMode {
    Elevation,
    Tectonics,
}

impl Default for HexWorldMapMode {
    fn default() -> Self {
        HexWorldMapMode::Tectonics
    }
}

pub type HexWorldQuery<'w, 's, 'c> = Query<
    'w,
    's,
    (
        Entity,
        &'c HexWorld,
        &'c Transform,
        &'c HexWorldMapMode,
        &'c WorldElevationData,
        &'c WorldTectonicsData,
    ),
>;

#[derive(Bundle, Default)]
pub struct HexWorldBundle {
    world: HexWorld,
    transform: Transform,
    map_mode: HexWorldMapMode,
    elevations: WorldElevationData,
    tectonics: WorldTectonicsData,
}

impl HexWorld {
    pub fn new() -> HexWorldBundle {
        HexWorldBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 500.)),
            ..Default::default()
        }
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}
