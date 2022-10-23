use bevy::{prelude::*, ui::Style};
use Val::*;

#[derive(Component, Debug)]
pub struct RootElement;

#[derive(Bundle)]
pub struct RootElementBundle {
    #[bundle]
    node_bundle: NodeBundle,
    root_element_marker: RootElement,
}

impl RootElementBundle {
    pub fn new() -> Self {
        RootElementBundle {
            node_bundle: get_root_node_bundle(),
            root_element_marker: RootElement,
        }
    }
}

fn get_root_node_bundle() -> NodeBundle {
    NodeBundle {
        color: UiColor::from(Color::NONE),

        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Px(0.)),
            size: Size::new(Percent(100.), Percent(100.)),
            margin: UiRect::all(Px(0.)),
            flex_grow: 1.0,
            ..default()
        },
        ..default()
    }
}
