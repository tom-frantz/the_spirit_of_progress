use crate::components::world::render::MainTileMap;
use bevy::prelude::*;

pub fn delete_main_tile_map(
    mut commands: &mut Commands,
    tile_maps: Query<Entity, With<MainTileMap>>,
) {
    for tile_map in tile_maps.iter() {
        commands.entity(tile_map).despawn_recursive();
    }
}
