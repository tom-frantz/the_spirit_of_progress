use crate::camera::MainCamera;
use crate::ui::RootElement;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub fn get_cursor_location(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform, With<MainCamera>)>,
) -> Option<Vec2> {
    let (camera, camera_transform, _) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        return Some(world_pos);
    }
    None
}

pub fn clear_ui_elements(
    commands: &mut Commands,
    q_ui_main_elements: &Query<Entity, With<RootElement>>,
) {
    for entity in q_ui_main_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
