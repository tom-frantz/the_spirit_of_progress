use crate::components::world::height::HeightMap;
// use crate::components::world::render::draw_map;
use crate::components::world::resources::GeographicWorldType::Height;
use crate::components::world::tectonics::TectonicPlates;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

pub enum GeographicWorldType {
    Height,
    TectonicPlates,
    TectonicPlateTypes,
}

pub struct GeographicWorld {
    currently_viewing: GeographicWorldType,

    height_map: HeightMap,
    tectonic_map: TectonicPlates,
}

impl GeographicWorld {
    pub fn new(height_map: HeightMap, tectonic_map: TectonicPlates) -> Self {
        GeographicWorld {
            currently_viewing: Height,

            height_map,
            tectonic_map,
        }
    }

    pub fn draw_world_type(&self, _commands: Commands, _asset_server: Res<AssetServer>) {
        match &self.currently_viewing {
            Height => {}
            GeographicWorldType::TectonicPlates => {}
            GeographicWorldType::TectonicPlateTypes => {}
        };
    }
}
