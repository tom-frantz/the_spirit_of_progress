use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use crate::latlon::ValuePoint;
use crate::tectonics::WorldTectonics;

pub mod tectonics;
pub mod latlon;

const PIXEL_SIZE: f32 = 4.;

fn get_tile(index: UVec2) -> ShapeBundle {
    let city_shape = shapes::Rectangle {
        extents: Vec2::new(PIXEL_SIZE, PIXEL_SIZE),
        origin: RectangleOrigin::BottomLeft,
    };

    GeometryBuilder::build_as(
        &city_shape,
        DrawMode::Fill(FillMode::color(Color::rgb(255.0, 0.0, 0.0))),
        Transform::from_xyz(index.x as f32, index.y as f32, 0.),
    )
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 360. * PIXEL_SIZE,
            height: 180. * PIXEL_SIZE,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin).add_startup_system(draw_map)
        .run()
}

fn draw_map(mut commands: Commands) {
    let world: WorldTectonics<f32> = WorldTectonics::new(0.5, 0.0, 0.0, vec![]);
}
