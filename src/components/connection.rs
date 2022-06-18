use crate::ui::interaction::{Clickable, InteractionEvents, MapClickable};
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
    map_clickable: MapClickable,
}

#[derive(Component)]
pub struct Connection {
    start_city: Entity,
    start_city_pos: Vec2,
    end_city: Entity,
    end_city_pos: Vec2,
}

impl Connection {
    pub fn new(start_city: (Entity, Transform), end_city: (Entity, Transform)) -> ConnectionBundle {
        let start_city_pos = start_city.1.translation.xy();
        let end_city_pos = end_city.1.translation.xy();

        let shape = shapes::Line(
            start_city_pos + Vec2::new(5.0, 5.0),
            end_city_pos + Vec2::new(5.0, 5.0),
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
                start_city_pos,
                end_city: end_city.0,
                end_city_pos,
            },
            map_clickable: default(),
        }
    }
}

impl Clickable for Connection {
    fn clicked(&self, transform: &Transform, mouse: Vec2) -> bool {
        point_shortest_distance_to_line(
            mouse - Vec2::new(5.0, 5.0),
            self.start_city_pos,
            self.end_city_pos,
        ) <= 5.0
    }

    fn event_type(&self, self_entity: Entity) -> InteractionEvents {
        InteractionEvents::Connection(self_entity)
    }
}

fn point_shortest_distance_to_line(point: Vec2, start: Vec2, end: Vec2) -> f32 {
    // https://stackoverflow.com/a/6853926

    let A = point.x - start.x;
    let B = point.y - start.y;
    let C = end.x - start.x;
    let D = end.y - start.y;

    let dot = A * C + B * D;
    let len_sq = C.powf(2.0) + D.powf(2.0);

    let mut param: f32 = -1.0;

    if len_sq != 0.0 {
        param = dot / len_sq
    }

    let xx;
    let yy;

    if param < 0.0 {
        xx = start.x;
        yy = start.y;
    } else if param > 1.0 {
        xx = end.x;
        yy = end.y;
    } else {
        xx = start.x + param * C;
        yy = start.y + param * D;
    }

    let dx = point.x - xx;
    let dy = point.y - yy;
    return (dx * dx + dy * dy).sqrt();
}
