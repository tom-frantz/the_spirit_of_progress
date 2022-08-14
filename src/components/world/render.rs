use crate::components::world::latlon::{ValuePoint, WorldPoint, LATITUDE_RANGE, LONGITUDE_RANGE};
use crate::components::world::{PIXEL_SIZE, TECTONIC_PRECISION};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::fmt::Debug;

#[derive(Component)]
pub struct MainTileMap;

pub trait WorldRender {
    // The handle to the asset to use for tiles
    fn texture_asset_name(&self) -> &str {
        "textured_tiles.png"
    }

    /// The size of an individual tile, in pixels, for the tile asset
    fn tile_size(&self) -> TilemapTileSize {
        TilemapTileSize {
            x: PIXEL_SIZE,
            y: PIXEL_SIZE,
        }
    }

    /// The size of the grid, in pixels, for the whole tile asset
    fn tilemap_asset_size(&self) -> TilemapGridSize {
        TilemapGridSize {
            x: PIXEL_SIZE * 4.,
            y: PIXEL_SIZE,
        }
    }

    /// The precision of the world's lat/lon delineation
    /// e.g. a precision of 2 would mean that lat/lon increases by .5 each step, for their ranges
    fn precision(&self) -> u32;

    /// The size of the world (lat/lon) in terms of tiles
    fn tilemap_size(&self) -> TilemapSize {
        TilemapSize {
            x: LONGITUDE_RANGE as u32 * self.precision(),
            y: LATITUDE_RANGE as u32 * self.precision() + 1,
        }
    }

    fn tilemap_transform(&self) -> Transform {
        bevy_ecs_tilemap::helpers::get_centered_transform_2d(
            &self.tilemap_size(),
            &self.tile_size(),
            0.0,
        )
    }

    fn bundle(&self, storage: TileStorage, asset_server: Res<AssetServer>) -> TilemapBundle {
        let handle = asset_server.load(self.texture_asset_name());
        println!("Handle: {}", self.texture_asset_name());

        TilemapBundle {
            // Size of the tile and the map rendered
            tile_size: self.tile_size(),
            size: self.tilemap_size(),
            storage,
            transform: self.tilemap_transform(),

            // asset information
            grid_size: self.tilemap_asset_size(),
            texture: TilemapTexture(handle),

            ..Default::default()
        }
    }
}

pub trait TileRender {
    type World;

    fn bundle(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileBundle {
        TileBundle {
            position,
            tilemap_id,
            color: self.color(world, position, tilemap_id),
            ..Default::default()
        }
    }

    fn color(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileColor {
        Default::default()
    }
}

impl<T> TileRender for ValuePoint<T>
where
    T: TileRender + Debug + Clone,
{
    type World = T::World;

    fn bundle(&self, world: &Self::World, position: TilePos, tilemap_id: TilemapId) -> TileBundle {
        self.value.bundle(world, position, tilemap_id)
    }
}

pub fn draw_map<WORLD, TILE>(world: &WORLD, mut commands: Commands, asset_server: Res<AssetServer>)
where
    WORLD: WorldRender,
    for<'a> &'a WORLD: IntoIterator<Item = &'a TILE>,
    TILE: TileRender<World = WORLD> + WorldPoint,
{
    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(world.tilemap_size());

    for point in world.into_iter() {
        let pos = TilePos {
            y: ((point.lat() + (LATITUDE_RANGE / 2.)) * TECTONIC_PRECISION as f32) as u32,
            x: ((point.lon() + (LONGITUDE_RANGE / 2.)) * TECTONIC_PRECISION as f32) as u32,
        };

        let tile_bundle = point.bundle(&world, pos.clone(), TilemapId(tilemap_entity));
        // println!("{:?}", tile_bundle);
        let tile_entity = commands.spawn().insert_bundle(tile_bundle).id();

        tile_storage.set(&pos, Some(tile_entity));
    }

    commands
        .entity(tilemap_entity)
        .insert_bundle(world.bundle(tile_storage, asset_server))
        .insert(MainTileMap);
}

/// Use this when you're losing your mind.
pub fn sanity_check(mut commands: Commands, asset_server: Res<AssetServer>) {
    let size = TilemapSize { x: 10, y: 10 };

    let tilemap_entity = commands.spawn().id();
    let mut tile_storage = TileStorage::empty(size);

    for x in 0..10 {
        for y in 0..10 {
            let pos = TilePos { x, y };

            let tile_bundle = TileBundle {
                position: pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            };

            let tile_entity = commands.spawn_bundle(tile_bundle).id();

            tile_storage.set(&pos, Some(tile_entity))
        }
    }

    let texture = asset_server.load("test_tile.png");

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            tile_size: TilemapTileSize { x: 12., y: 12. },
            size,
            storage: tile_storage,

            grid_size: TilemapGridSize { x: 12., y: 12. },
            texture: TilemapTexture(texture),
            ..Default::default()
        });
}
