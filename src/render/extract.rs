use crate::{
    game::world::{HexWorld, HexWorldData, HexWorldMapMode, HexWorldQuery},
    render::traits::QueryCellRender,
};
use bevy::{prelude::*, render::Extract};
use h3ron::H3Cell;

#[derive(Component)]
pub struct ExtractedHexWorld(pub HexWorld);
#[derive(Component)]
pub struct ExtractedHexWorldMapMode(pub HexWorldMapMode);
#[derive(Component)]
pub struct ExtractedHexWorldData<T>(pub T)
where
    T: QueryCellRender;

impl<T> QueryCellRender for ExtractedHexWorldData<T>
where
    T: QueryCellRender,
{
    fn cell_colour(&self, cell_id: H3Cell) -> Color {
        self.0.cell_colour(cell_id)
    }
}

pub fn extract(mut commands: Commands, hex_world: Extract<HexWorldQuery>) {
    for (entity, hex_world, transform, map_mode, elevation_data, tectonics_data) in hex_world.iter() {
        let mut entity_commands = commands.spawn();
        entity_commands
            .insert(ExtractedHexWorld(*hex_world))
            .insert(transform.clone())
            .insert(ExtractedHexWorldMapMode(*map_mode));

        match map_mode {
            HexWorldMapMode::Elevation => {
                println!("{elevation_data:?}");
                entity_commands.insert(ExtractedHexWorldData(elevation_data.clone()));
            }
            HexWorldMapMode::Tectonics => {
                entity_commands.insert(ExtractedHexWorldData(tectonics_data.clone()));
            }
        };
    }
}
