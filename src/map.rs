use crate::ui::theme::{Colour, Terrain};
use crate::utils::rendering::ZIndex;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

pub const MAP_SIZE: f32 = 1000.0;

pub fn create_map() -> ShapeBundle {
    let map_shape = shapes::Rectangle {
        extents: Vec2::new(MAP_SIZE, MAP_SIZE),
        origin: RectangleOrigin::BottomLeft,
    };
    GeometryBuilder::build_as(
        &map_shape,
        DrawMode::Fill(FillMode::color(Terrain::SeaLevelLand.color())),
        Transform::from_xyz(0.0, 0.0, ZIndex::Map.into()),
    )
}
