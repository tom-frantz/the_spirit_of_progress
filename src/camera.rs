use crate::game::world::HexWorld;
use bevy::prelude::*;
use std::f32::consts::PI;

const ZOOM_OUT_KEYCODE: KeyCode = KeyCode::X;
const ZOOM_IN_KEYCODE: KeyCode = KeyCode::Z;

const ROTATE_X_KEYCODE: KeyCode = KeyCode::Q;
const ROTATE_COUNTER_X_KEYCODE: KeyCode = KeyCode::E;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera)
            .add_system(camera_move_system)
            .add_system(camera_rotate_system)
            .add_system(camera_zoom_system);
    }
}

pub fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}

pub fn camera_zoom_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut transform = query.get_single_mut().unwrap();
    let mut scale = transform.scale;

    if keyboard_input.just_pressed(ZOOM_OUT_KEYCODE) {
        scale *= 2.;
    }

    if keyboard_input.just_pressed(ZOOM_IN_KEYCODE) {
        scale /= 2.;
    }

    // println!("{}", scale);

    transform.scale = scale
}

pub fn camera_rotate_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform), With<HexWorld>>,
) {
    let mut transform = query.get_single_mut().unwrap();

    if keyboard_input.pressed(ROTATE_X_KEYCODE) {
        transform.rotate_y(PI / 60.);
    }

    if keyboard_input.pressed(ROTATE_COUNTER_X_KEYCODE) {
        transform.rotate_y(-PI / 60.);
    }

    println!("{:?}", transform.rotation);
}

pub fn camera_move_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
) {
    let speed = 1.0;
    let mut movement = Transform::default();
    let (mut transform, projection) = query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::W) {
        movement.translation += Vec3::new(0.0, speed * projection.scale, 0.0)
    }

    if keyboard_input.pressed(KeyCode::A) {
        movement.translation += Vec3::new(-speed * projection.scale, 0.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement.translation += Vec3::new(0.0, -speed * projection.scale, 0.0)
    }

    if keyboard_input.pressed(KeyCode::D) {
        movement.translation += Vec3::new(speed * projection.scale, 0.0, 0.0)
    }

    transform.translation += Vec3::splat(time.delta().as_millis() as f32) * movement.translation
}
