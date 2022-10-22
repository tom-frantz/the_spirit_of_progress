use crate::ui::{
    primitives::header::render_header,
    theme::{Colour, MenuColour, SPACING},
    RootElement::CenterBox,
};
use bevy::prelude::{Val::*, *};

#[derive(Default)]
pub struct CenterBoxProps {
    pub bare: bool,
}

pub fn render_center_box<T>(props: CenterBoxProps, parent: &mut ChildBuilder, spawn_children: T)
where
    T: FnOnce(&mut ChildBuilder) -> (),
{
    parent
        .spawn_bundle(background_bundle())
        .insert(CenterBox)
        .with_children(|center_box| {
            // render_header(center_box);
            center_box
                .spawn_bundle(content_bundle(props.bare))
                .with_children(spawn_children);
        });
}

fn background_bundle() -> NodeBundle {
    const _CENTER_BOX_SIZE: f32 = 80.0;
    NodeBundle {
        color: MenuColour::BorderBackground.ui_color(),

        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Px(SPACING)),
            size: Size::new(Auto, Auto),
            margin: UiRect::all(Px(50.0)),
            flex_grow: 1.0,
            display: Display::Flex,
            ..default()
        },
        ..Default::default()
    }
}

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
