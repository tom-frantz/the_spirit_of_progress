use crate::utils::colours::MapColour;
use crate::utils::rendering::ZIndex;
use crate::Keyframes::Translation;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_lyon::draw::{DrawMode, FillMode, StrokeMode};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::shapes;

#[derive(Bundle)]
pub struct ConnectionBundle {
    #[bundle]
    shape: ShapeBundle,

    connection: Connection,
}

#[derive(Component)]
pub struct Connection {
    start_city: Entity,
    end_city: Entity,
}

impl Connection {
    pub fn new(start_city: (Entity, Transform), end_city: (Entity, Transform)) -> ConnectionBundle {
        let start_translation = start_city.1.translation;
        let end_translation = end_city.1.translation;

        let shape = shapes::Line(
            start_translation.xy() + Vec2::new(5.0, 5.0),
            end_translation.xy() + Vec2::new(5.0, 5.0),
        );

        let mut color: Color = MapColour::DarkOrange.into();
        color.set_a(0.5);

        ConnectionBundle {
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Stroke(StrokeMode::new(color, 5.0)),
                Transform::from_xyz(0.0, 0.0, ZIndex::Connection.into()),
            ),
            connection: Connection {
                start_city: start_city.0,
                end_city: end_city.0,
            },
        }
    }
}
