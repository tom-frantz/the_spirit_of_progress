use crate::ui::primitives::header::render_header;
use crate::ui::theme::TypographyColour;
use crate::ui::theme::SPACING;
use crate::ui::MainElements::CenterBox;
use bevy::prelude::Val::*;
use bevy::prelude::*;

pub fn render_center_box<T>(parent: &mut ChildBuilder, spawn_children: T)
where
    T: FnOnce(&mut ChildBuilder) -> (),
{
    parent
        .spawn_bundle(background_bundle())
        .insert(CenterBox)
        .with_children(|center_box| {
            render_header(center_box);
            center_box
                .spawn_bundle(content_bundle())
                .with_children(spawn_children);
        });
}

fn background_bundle() -> NodeBundle {
    const CENTER_BOX_SIZE: f32 = 80.0;
    NodeBundle {
        color: TypographyColour::BackgroundBorder.into(),

        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Px(SPACING)),
            size: Size::new(Percent(CENTER_BOX_SIZE), Auto),
            margin: Rect::all(Px(50.0)),
            flex_grow: 1.0,
            display: Display::Flex,
            ..default()
        },
        ..Default::default()
    }
}

fn content_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::Background.into(),
        style: Style {
            size: Size::new(Percent(100.0), Auto),
            flex_grow: 1.0,
            ..default()
        },
        ..Default::default()
    }
}
