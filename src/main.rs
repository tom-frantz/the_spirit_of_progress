use crate::camera::camera_move_system;
use crate::city::{City, CityBundle};
use crate::connection::Connection;
use crate::map::create_map;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::mem::transmute;

mod city;
mod connection;

mod map;

mod camera;
pub mod utils;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(camera_move_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(create_map());

    let milbourne = commands
        .spawn_bundle(City::new("Milbourne".to_string(), Vec2::new(400.0, 100.0)))
        .id();

    let moodend = commands
        .spawn_bundle(City::new("Moodend".to_string(), Vec2::new(200.0, 200.0)))
        .id();

    commands.spawn_bundle(Connection::new(
        (milbourne, Transform::from_xyz(400.0, 100.0, 0.0)),
        (moodend, Transform::from_xyz(200.0, 200.0, 0.0)),
    ));
}
