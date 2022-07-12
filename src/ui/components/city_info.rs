use crate::ui::fonts::Typography;
use crate::ui::{LabelledNodeBundle, MainElements};

use crate::City;

use crate::components::city::CityComponents;
use crate::utils::colours::TypographyColour;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CityInfo;

pub fn create_city_info(
    city: Entity,
    city_query: &CityComponents,
    asset_server: &AssetServer,
    parent: &mut ChildBuilder,
) {
    let city = city_query.get(city).unwrap();

    parent
        .spawn_bundle(container_node())
        .with_children(|container_node| {
            container_node.spawn_bundle(city_name(city, asset_server));
        });
}

fn container_node() -> LabelledNodeBundle<MainElements> {
    LabelledNodeBundle {
        node_bundle: NodeBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                ..default()
            },
            color: TypographyColour::Background.into(),
            ..default()
        },
        label: MainElements::CityInfo(CityInfo {}),
    }
}

fn city_name(city: &City, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        text: Typography::Title.with_section(&city.name, default(), default(), asset_server),
        ..default()
    }
}
