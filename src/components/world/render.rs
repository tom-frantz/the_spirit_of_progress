use crate::{
    WorldPoint, WorldPoints, WorldTectonicsIndex, LATITUDE_RANGE, LONGITUDE_RANGE, PIXEL_SIZE,
    TECTONIC_PRECISION,
};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

fn get_tile_bundle(lat: f32, lon: f32, value: f32, tilemap_id: TilemapId) -> TileBundle {
    TileBundle {
        position: TilePos {
            y: ((lat + (LATITUDE_RANGE / 2.)) * TECTONIC_PRECISION as f32) as u32,
            x: ((lon + (LONGITUDE_RANGE / 2.)) * TECTONIC_PRECISION as f32) as u32,
        },
        tilemap_id,
        ..Default::default()
    }
}

pub fn draw_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let world: WorldPoints<f32> = WorldPoints::new(TECTONIC_PRECISION, |index| match index {
        WorldTectonicsIndex::NorthPole => 0.0,
        WorldTectonicsIndex::SouthPole => 0.0,
        WorldTectonicsIndex::Point(point) => point.lat() + point.lon(),
    });

    // Logging to see progress
    let mut last_point: Option<f32> = None;

    let tilemap_size = TilemapSize {
        x: LONGITUDE_RANGE as u32 * TECTONIC_PRECISION,
        y: LATITUDE_RANGE as u32 * TECTONIC_PRECISION + 1,
    };

    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for point in world.into_iter() {
        // Just logging to see progress
        if let Some(lat) = last_point {
            if (point.lat() - lat).abs() > 0.01 {
                println!("lat: {}", point.lat());
                last_point = Some(point.lat());
            }
        } else {
            println!("lat: {}", point.lat());
            last_point = Some(point.lat());
        }

        let tile_bundle = get_tile_bundle(
            point.latitude(),
            point.longitude(),
            point.value,
            TilemapId(tilemap_entity),
        );
        let tile_pos = tile_bundle.position.clone();
        let tile_entity = commands.spawn().insert_bundle(tile_bundle).id();

        tile_storage.set(&tile_pos, Some(tile_entity));
    }

    let texture_handle: Handle<Image> = asset_server.load("pergamon_tiles.png");

    let tile_size = TilemapTileSize {
        x: PIXEL_SIZE,
        y: PIXEL_SIZE,
    };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            tile_size,
            grid_size: TilemapGridSize {
                x: PIXEL_SIZE,
                y: PIXEL_SIZE * 8.,
            },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                0.0,
            ),
            ..Default::default()
        });
}
