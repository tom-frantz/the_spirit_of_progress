use crate::latlon::{LatLonPoint, ValuePoint, WorldPoint, LATITUDE_RANGE, LONGITUDE_RANGE};
use crate::tectonics::utils::WorldTectonicsIndex;
use crate::tectonics::WorldTectonics;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy_ecs_tilemap::prelude::TilemapGridSize;
use bevy_ecs_tilemap::TilemapBundle;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

pub mod latlon;
pub mod tectonics;

const PIXEL_SIZE: f32 = 3.;
const PIXEL_BUFFER: f32 = 5.;
const TECTONIC_PRECISION: f32 = 2.;

fn get_tile(lat: f32, lon: f32, value: f32) -> ShapeBundle {
    let city_shape = if lat == 90. || lat == -90. {
        let x = PIXEL_SIZE * TECTONIC_PRECISION * LATITUDE_RANGE;
        shapes::Rectangle {
            extents: Vec2::new(x, PIXEL_SIZE),
            origin: RectangleOrigin::CustomCenter(Vec2::new(PIXEL_SIZE, PIXEL_SIZE / 2.)),
        }
    } else {
        shapes::Rectangle {
            extents: Vec2::new(PIXEL_SIZE, PIXEL_SIZE),
            origin: RectangleOrigin::BottomLeft,
        }
    };

    GeometryBuilder::build_as(
        &city_shape,
        DrawMode::Fill(FillMode::color(Color::rgb(
            (value + 270.0) / (270.0 * TECTONIC_PRECISION),
            0.0,
            0.0,
        ))),
        Transform::from_xyz(
            (lon) * TECTONIC_PRECISION * PIXEL_SIZE,
            (lat) * TECTONIC_PRECISION * PIXEL_SIZE,
            0.,
        ),
    )
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: (LONGITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION * PIXEL_SIZE,
            height: (LATITUDE_RANGE + PIXEL_BUFFER) * TECTONIC_PRECISION * PIXEL_SIZE,
            ..default()
        })
        .insert_resource(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(draw_map)
        .run()
}

fn draw_map(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let world: WorldTectonics<f32> =
        WorldTectonics::new_with_func(TECTONIC_PRECISION, |index| match index {
            WorldTectonicsIndex::NorthPole => 0.0,
            WorldTectonicsIndex::SouthPole => 0.0,
            WorldTectonicsIndex::Point(point) => point.lat() + point.lon(),
        });
    let mut last_point: Option<f32> = None;

    let map = world.into_iter().map(move |point| {
        if let Some(lat) = last_point {
            if (point.lat() - lat).abs() > 0.01 {
                println!("lat: {}", point.lat());
                last_point = Some(point.lat());
            }
        } else {
            println!("lat: {}", point.lat());
            last_point = Some(point.lat());
        }

        let tile = get_tile(point.latitude(), point.longitude(), point.value);
        tile
    });
    commands.spawn_batch(map);
}

// -1 to 1
// -0.5, 0, .5, 1
