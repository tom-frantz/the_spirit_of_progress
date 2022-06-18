use crate::ui::fonts::Typography;
use crate::ui::LabelledNodeBundle;

use crate::City;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component)]
pub struct CityInfoLabel;

fn container_node() -> LabelledNodeBundle<CityInfoLabel> {
    LabelledNodeBundle {
        node_bundle: NodeBundle {
            style: Style {
                size: Size::new(Val::Px(500.0), Val::Px(100.0)),
                ..default()
            },
            ..default()
        },
        label: CityInfoLabel,
    }
}

fn city_name(city: &City, asset_server: &AssetServer) -> TextBundle {
    TextBundle {
        text: Typography::Title.with_section(&city.name, default(), default(), asset_server),
        ..default()
    }
}
