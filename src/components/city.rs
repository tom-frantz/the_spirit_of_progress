use crate::ui::interaction::{Clickable, MapClickable, MapInteractionEvents};
use crate::ui::theme::{Colour, MenuColour};
use crate::utils::rendering::ZIndex;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_prototype_lyon::draw::{DrawMode, FillMode};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::{GeometryBuilder, RectangleOrigin};
use bevy_prototype_lyon::shapes;

#[derive(Component)]
pub struct City {
    pub name: String,
}

const CITY_SHAPE_SIZE: f32 = 10.0;

impl City {
    pub fn new(name: String, location: Vec2) -> CityBundle {
        let city_shape = shapes::Rectangle {
            extents: Vec2::new(CITY_SHAPE_SIZE, CITY_SHAPE_SIZE),
            origin: RectangleOrigin::BottomLeft,
        };

        CityBundle {
            city: City { name },
            map_clickable: MapClickable,
            shape: GeometryBuilder::build_as(
                &city_shape,
                DrawMode::Fill(FillMode::color(MenuColour::BlackPen.color())),
                Transform::from_xyz(location.x, location.y, ZIndex::City.into()),
            ),
        }
    }
}

impl Clickable for City {
    fn clicked(&self, self_transform: &Transform, cursor: Vec2) -> bool {
        let difference = cursor - self_transform.translation.xy();

        difference.x >= 0.0
            && difference.x < CITY_SHAPE_SIZE
            && difference.y >= 0.0
            && difference.y < CITY_SHAPE_SIZE
    }

    fn event_type(&self, self_entity: Entity) -> MapInteractionEvents {
        MapInteractionEvents::City(self_entity)
    }
}

pub enum CityEvents {
    Created,
    Destroyed,
}

pub type CityComponents<'w, 's, 'a> = Query<'w, 's, &'a City>;

#[derive(Bundle)]
pub struct CityBundle {
    pub city: City,
    pub map_clickable: MapClickable,
    // This includes a transform.
    #[bundle]
    pub shape: ShapeBundle,
}

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CityEvents>();
    }
}
