use crate::game::world::HexWorld;
use bevy::{prelude::*, time::FixedTimestep};
use std::f64::consts::TAU;

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;

const ZOOM_OUT_KEYCODE: KeyCode = KeyCode::X;
const ZOOM_IN_KEYCODE: KeyCode = KeyCode::Z;

const ROTATE_COUNTER_LON_KEYCODE: KeyCode = KeyCode::A;
const ROTATE_LON_KEYCODE: KeyCode = KeyCode::D;

const ROTATE_LAT_KEYCODE: KeyCode = KeyCode::W;
const ROTATE_COUNTER_LAT_KEYCODE: KeyCode = KeyCode::S;

#[derive(Component)]
pub struct MainCamera;

#[derive(Default, Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera)
            // .add_system(camera_move_system)
            .add_system_set(
                SystemSet::new().with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND)), // .with_system(debug_camera_state),
            )
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
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
    mut query: Query<&mut Transform, With<HexWorld>>,
) {
    let mut transform = query.get_single_mut().unwrap();
    let scale = camera_query.get_single().unwrap().scale;

    let speed = TAU as f32 / 60. / 4. * scale;
    let prev_y = transform.local_y().y;

    if keyboard_input.pressed(ROTATE_LON_KEYCODE) {
        transform.rotate_local_y(speed);
    }

    if keyboard_input.pressed(ROTATE_COUNTER_LON_KEYCODE) {
        transform.rotate_local_y(-speed);
    }

    if keyboard_input.pressed(ROTATE_LAT_KEYCODE) {
        transform.rotate_x(-speed);

        // HACK: Correct the rotation after the fact.
        // Quaternions are weird yo.
        let current_y = transform.local_y().y;
        if current_y < prev_y && current_y < 0. {
            transform.rotate_x(speed);
        }
    }

    if keyboard_input.pressed(ROTATE_COUNTER_LAT_KEYCODE) {
        transform.rotate_x(speed);

        // Correct after the fact
        let current_y = transform.local_y().y;
        if current_y < prev_y && current_y < 0. {
            transform.rotate_x(-speed);
        }
    }
}

#[deprecated]
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

fn debug_camera_state(mut query: Query<&mut Transform, With<HexWorld>>) {
    let transform = query.get_single_mut().unwrap();

    println!("Up: {:#?}, Down: {:#?}", transform.up(), transform.down());
}
