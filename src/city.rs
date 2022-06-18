use crate::utils::colours::MapColour;
use crate::utils::rendering::ZIndex;
use bevy::prelude::*;
use bevy::reflect::ReflectRef::Map;
use bevy_prototype_lyon::draw::{DrawMode, FillMode};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::{GeometryBuilder, RectangleOrigin};
use bevy_prototype_lyon::shapes;

#[derive(Bundle)]
pub struct CityBundle {
    pub city: City,

    // This includes a transform.
    #[bundle]
    pub shape: ShapeBundle,
}

#[derive(Component)]
pub struct City {
    pub name: String,
}

impl City {
    pub fn new(name: String, location: Vec2) -> CityBundle {
        let city_shape = shapes::Rectangle {
            extents: Vec2::new(10.0, 10.0),
            origin: RectangleOrigin::BottomLeft,
        };

        CityBundle {
            city: City { name },
            shape: GeometryBuilder::build_as(
                &city_shape,
                DrawMode::Fill(FillMode::color(MapColour::Dark.into())),
                Transform::from_xyz(location.x, location.y, ZIndex::City.into()),
            ),
        }
    }
}
