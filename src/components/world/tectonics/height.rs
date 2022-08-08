use bevy_ecs_tilemap::prelude::TileBundle;

pub struct HeightData {
    height: f32,
}

impl HeightData {
    pub fn new(height: f32) -> Self {
        HeightData { height }
    }
}
