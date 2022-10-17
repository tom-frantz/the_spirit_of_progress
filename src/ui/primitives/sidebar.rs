use crate::ui::primitives::header::render_header;
use crate::ui::theme::SPACING;
use crate::ui::theme::{Colour, MenuColour};
use crate::ui::RootElement::Sidebar;
use bevy::prelude::Val::*;
use bevy::prelude::*;

pub fn render_sidebar<T>(parent: &mut ChildBuilder, spawn_children: T)
where
    T: FnOnce(&mut ChildBuilder) -> (),
{
    parent
        .spawn_bundle(background_bundle())
        .insert(Sidebar)
        .with_children(|sidebar| {
            render_header(sidebar);
            sidebar
                .spawn_bundle(content_bundle())
                .with_children(spawn_children);
        });
}

const SIDEBAR_BACKGROUND_WIDTH: f32 = 800.0;
const SIDEBAR_BACKGROUND_PADDING: f32 = SPACING;
const SIDEBAR_CONTENT_PADDING: f32 = SPACING;
pub const SIDEBAR_CONTENT_SIZE: f32 =
    SIDEBAR_BACKGROUND_WIDTH - SIDEBAR_BACKGROUND_PADDING * 2.0 - SIDEBAR_CONTENT_PADDING * 2.0;

fn background_bundle() -> NodeBundle {
    NodeBundle {
        color: MenuColour::BorderBackground.ui_color(),

        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Px(SIDEBAR_BACKGROUND_PADDING)),
            size: Size::new(Px(SIDEBAR_BACKGROUND_WIDTH), Percent(100.0)),
            ..default()
        },
        ..Default::default()
    }
}

fn content_bundle() -> NodeBundle {
    NodeBundle {
        color: MenuColour::Background.ui_color(),
        style: Style {
            size: Size::new(Percent(100.0), Auto),
            flex_grow: 1.0,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Px(SIDEBAR_CONTENT_PADDING)),
            ..default()
        },
        ..Default::default()
    }
}
