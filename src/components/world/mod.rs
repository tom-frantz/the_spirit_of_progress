use crate::latlon::{LatLonPoint, ValuePoint, WorldPoint};
use crate::tectonics::WorldTectonics;
use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

pub mod latlon;
pub mod tectonics;

const PIXEL_SIZE: f32 = 2.;

fn get_tile(lat: f32, lon: f32, value: f32) -> ShapeBundle {
    let city_shape = shapes::Rectangle {
        extents: Vec2::new(PIXEL_SIZE, PIXEL_SIZE),
        origin: RectangleOrigin::BottomLeft,
    };

    let LATLON_PERCISION = 2.0;
    GeometryBuilder::build_as(
        &city_shape,
        DrawMode::Fill(FillMode::color(Color::rgb(
            (value + 270.0) / (270.0 * LATLON_PERCISION),
            0.0,
            0.0,
        ))),
        Transform::from_xyz(
            (lat) * LATLON_PERCISION * PIXEL_SIZE,
            (lon) * LATLON_PERCISION * PIXEL_SIZE,
            0.,
        ),
    )
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 360. * 2.0 * PIXEL_SIZE,
            height: 180. * 2.0 * PIXEL_SIZE,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(draw_map)
        .run()
}

fn draw_map(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // .insert(MainCamera);

    let mut points = HashMap::with_capacity(360 * 720);

    for mut lat_index in 0..720 {
        let lat = (lat_index as f32 + 1.0) / 2.0 - 180.0;

        'label: for mut lon_index in 0..360 {
            let lon = (lon_index as f32 + 1.0) / 2.0 - 90.0;

            let point = LatLonPoint::new(lat, lon);
            points.insert(point, ValuePoint::new(point, lat + lon));

            if lon_index == 0 || lon_index == 360 {
                continue 'label;
            }
        }
    }

    let world: WorldTectonics<f32> = WorldTectonics::new(0.5, 0.0, 0.0, points);
    let mut last_point: Option<f32> = None;
    for point in world.iter() {
        if let Some(lon) = last_point {
            if point.lon() - lon < 0.01 {
                println!("LON: {}", point.lon());
                last_point = Some(lon);
            }
        } else {
            println!("LON: {}", point.lon());
            last_point = Some(point.lon());
        }
        if point.lat() == 179.5
            || point.lat() == -179.5
            || point.lon() == 89.5
            || point.lon() == -89.5
        {
            let tile = get_tile(point.latitude(), point.longitude(), point.value);
            commands.spawn_bundle(tile);
        }
    }
}

// -1 to 1
// -0.5, 0, .5, 1
