use crate::components::world::height::{HeightMap, HeightPoint};
// use crate::components::world::render::draw_map;
use crate::components::world::render::events::{MapModeEvent, ViewMode};
use crate::components::world::render::helpers::delete_main_tile_map;
use crate::components::world::render::{MainTileMap, RenderTheWorld};
use crate::components::world::tectonics::plate::PlateType;
use crate::components::world::tectonics::point::PlatePoint;
use crate::components::world::tectonics::TectonicsMap;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub struct GeographicWorld {
    currently_viewing: ViewMode,

    height_map: HeightMap,
    tectonic_map: TectonicsMap,
}

impl GeographicWorld {
    pub fn new(height_map: HeightMap, tectonic_map: TectonicsMap) -> Self {
        GeographicWorld {
            currently_viewing: ViewMode::TectonicPlates,

            height_map,
            tectonic_map,
        }
    }

    pub fn draw_world_type(
        &self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        tile_maps: Query<Entity, With<MainTileMap>>,
    ) {
        delete_main_tile_map(&mut commands, tile_maps);
        match &self.currently_viewing {
            ViewMode::Heights => {
                HeightPoint::render_world(&self.height_map, commands, asset_server)
            }
            ViewMode::TectonicPlates => {
                PlatePoint::render_world(&self.tectonic_map, commands, asset_server)
            }
            ViewMode::TectonicPlateTypes => {
                PlateType::render_world(&self.tectonic_map, commands, asset_server)
            }
        };
    }

    pub fn handle_map_mode_event(
        mut geo_world: ResMut<GeographicWorld>,
        mut map_mode_events: EventReader<MapModeEvent>,
        commands: Commands,
        asset_server: Res<AssetServer>,
        tile_maps: Query<Entity, With<MainTileMap>>,
    ) {
        let old = geo_world.currently_viewing;
        for event in map_mode_events.iter() {
            match event {
                MapModeEvent::SetViewMode(view_mode) => {
                    geo_world.currently_viewing = *view_mode;
                }
            }
        }
        if old != geo_world.currently_viewing {
            geo_world.draw_world_type(commands, asset_server, tile_maps)
        }
    }
}

pub fn handle_map_mode_change(
    keyboard_input: Res<Input<KeyCode>>,
    mut map_mode_event_writer: EventWriter<MapModeEvent>,
) {
    for key_code in keyboard_input.get_just_pressed() {
        match key_code {
            KeyCode::Key1 => {
                map_mode_event_writer.send(MapModeEvent::SetViewMode(ViewMode::Heights))
            }
            KeyCode::Key2 => {
                map_mode_event_writer.send(MapModeEvent::SetViewMode(ViewMode::TectonicPlates))
            }
            KeyCode::Key3 => {
                map_mode_event_writer.send(MapModeEvent::SetViewMode(ViewMode::TectonicPlateTypes))
            }
            _ => {}
        }
    }
}
