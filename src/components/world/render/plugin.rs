use crate::components::world::height::{HeightMap, HeightPoint};
use crate::components::world::render::events::MapModeEvent;
use crate::components::world::render::resources::{handle_map_mode_change, GeographicWorld};
use crate::components::world::tectonics::TectonicsMap;
use crate::components::world::WorldPoints;
use bevy::prelude::*;

pub struct WorldRenderPlugin;

impl Plugin for WorldRenderPlugin {
    fn build(&self, app: &mut App) {
        // TODO move these to a better spot
        let heightmap = HeightMap::new(WorldPoints::new(2, |_p| HeightPoint::new(0.0)));
        let tectonics = TectonicsMap::new(2, 2, 9);

        app.add_event::<MapModeEvent>()
            .insert_resource(GeographicWorld::new(heightmap, tectonics))
            .add_system(GeographicWorld::handle_map_mode_event)
            .add_system(handle_map_mode_change);
    }
}
