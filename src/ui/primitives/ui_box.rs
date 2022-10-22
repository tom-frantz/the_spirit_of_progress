use crate::ui::theme::{Colour, MenuColour};
use bevy::prelude::{Val::*, *};

fn content_bundle(bare: bool) -> NodeBundle {
    let color = if bare {
        UiColor::from(Color::NONE)
    } else {
        MenuColour::Background.ui_color()
    };

    NodeBundle {
        color,
        style: Style {
            padding: UiRect::all(Px(8.)),
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::FlexStart,
            size: Size::new(Percent(100.0), Auto),
            flex_grow: 1.0,
            ..default()
        },
        ..Default::default()
    }
}
