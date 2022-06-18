use bevy::prelude::*;

pub fn camera_move_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let mut movement = Transform::default();
    let (mut transform) = query.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::W) {
        movement.translation += Vec3::new(0.0, 1.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::A) {
        movement.translation += Vec3::new(-1.0, 0.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement.translation += Vec3::new(0.0, -1.0, 0.0)
    }

    if keyboard_input.pressed(KeyCode::D) {
        movement.translation += Vec3::new(1.0, 0.0, 0.0)
    }

    transform.translation += Vec3::splat(time.delta().as_millis() as f32) * movement.translation
}
