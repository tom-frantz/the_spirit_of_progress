use bevy::prelude::*;
use bevy::ui::Style;
use bevy::window::CursorIcon::Default;
use Val::*;

pub fn get_root_node_bundle() -> NodeBundle {
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