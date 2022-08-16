use crate::components::world::latlon::{LatLonPoint, ValuePoint, LATITUDE_RANGE, LONGITUDE_RANGE};
use crate::components::world::utils::iterators::WorldPointsIterator;
use crate::components::world::{WorldPoints, PIXEL_SIZE};
use bevy::prelude::Commands;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::fmt::Debug;

#[derive(Component)]
pub struct MainTileMap;

pub struct ExampleWorld {
    pub world: WorldPoints<ExampleWorldPoint>,
}

#[derive(Debug, Clone)]
pub struct ExampleWorldPoint {}

pub trait RenderTheWorld<'a> {
    type World: WorldMap<'a>;

    fn get_tile_bundle(
        point: &<Self::World as WorldMap<'a>>::Point,
        world: &Self::World,
        tilemap_id: TilemapId,
    ) -> TileBundle;

    // fn get_tilemap_bundle(world: &Self::World) -> TilemapBundle;

    fn render_world(
        world: &'a <Self as RenderTheWorld<'a>>::World,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let tilemap_entity = commands.spawn().id();
        let mut tile_storage = TileStorage::empty(world.map_size());

        for point in world.iter_points() {
            let bundle = Self::get_tile_bundle(&point, world, TilemapId(tilemap_entity));

            let tile_entity = commands.spawn().insert_bundle(bundle).id();

            tile_storage.set(&bundle.position, Some(tile_entity));
        }

        commands
            .entity(tilemap_entity)
            .insert_bundle(world.bundle(tile_storage, asset_server))
            .insert(MainTileMap);
    }
}

pub trait WorldMap<'a> {
    type PointsIterator: Iterator<Item = Self::Point>;
    type Point;

    fn iter_points(&'a self) -> Self::PointsIterator;

    fn precision(&self) -> u32;

    fn tile_size(&self) -> TilemapTileSize {
        TilemapTileSize {
            x: PIXEL_SIZE,
            y: PIXEL_SIZE,
        }
    }

    fn map_size(&self) -> TilemapSize {
        TilemapSize {
            x: LONGITUDE_RANGE as u32 * self.precision(),
            y: LATITUDE_RANGE as u32 * self.precision() + 1,
        }
    }

    /// i.e. the size of a file from the `assets` folder.
    fn asset_size(&self) -> TilemapGridSize {
        TilemapGridSize {
            x: PIXEL_SIZE * 4.,
            y: PIXEL_SIZE,
        }
    }

    fn asset_name(&self) -> &str {
        "textured_tiles.png"
    }

    fn map_centered_transform(&self) -> Transform {
        bevy_ecs_tilemap::helpers::get_centered_transform_2d(
            &self.map_size(),
            &self.tile_size(),
            0.0,
        )
    }

    fn bundle(&self, storage: TileStorage, asset_server: Res<AssetServer>) -> TilemapBundle {
        let handle = asset_server.load(self.asset_name());

        TilemapBundle {
            // Size of the tile and the map rendered
            tile_size: self.tile_size(),
            size: self.map_size(),
            storage,
            transform: self.map_centered_transform(),

            // asset information
            grid_size: self.asset_size(),
            texture: TilemapTexture(handle),

            ..Default::default()
        }
    }
}

impl<'world_map, T> WorldMap<'world_map> for WorldPoints<T>
where
    T: Debug + Clone + 'world_map,
{
    type PointsIterator = WorldPointsIterator<'world_map, T>;
    type Point = &'world_map ValuePoint<T>;

    fn iter_points(&'world_map self) -> Self::PointsIterator {
        self.iter()
    }

    fn precision(&self) -> u32 {
        self.precision
    }
}

impl<'world_map, T> WorldMap<'world_map> for &'world_map WorldPoints<T>
where
    T: Debug + Clone + 'world_map,
{
    type PointsIterator = WorldPointsIterator<'world_map, T>;
    type Point = &'world_map ValuePoint<T>;

    fn iter_points(&self) -> Self::PointsIterator {
        self.iter()
    }

    fn precision(&self) -> u32 {
        self.precision
    }
}

pub trait World {
    type Point: Clone + Debug;
    fn get_world(&self) -> &WorldPoints<Self::Point>;
}

impl<'a, T, P> WorldMap<'a> for &'a T
where
    T: World<Point = P>,
    P: Debug + Clone + 'a,
{
    type PointsIterator = WorldPointsIterator<'a, P>;
    type Point = &'a ValuePoint<P>;

    fn iter_points(&self) -> Self::PointsIterator {
        self.get_world().iter()
    }

    fn precision(&self) -> u32 {
        self.get_world().precision
    }
}
