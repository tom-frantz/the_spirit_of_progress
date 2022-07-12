use crate::utils::colours::TypographyColour;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct SidebarLabel;

pub fn sidebar_builder(parent: &mut ChildBuilder) {
    let mut sidebar_commands = parent.spawn_bundle(background_bundle());

    sidebar_commands
        .insert(SidebarLabel)
        .with_children(|sidebar| {
            sidebar
                .spawn_bundle(header_bundle())
                .with_children(|header| {
                    header.spawn_bundle(close_button_bundle());
                });
            sidebar.spawn_bundle(content_bundle());
        });

    sidebar_commands;
}

fn background_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::BackgroundBorder.into(),

        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(8.0)),
            size: Size::new(Val::Px(400.0), Val::Percent(100.0)),
            ..default()
        },
        ..Default::default()
    }
}

fn header_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::Background.into(),
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(16.0)),
            justify_content: JustifyContent::FlexEnd,
            margin: Rect {
                bottom: Val::Px(8.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}

fn close_sidebar_button_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::Red.into(),
        style: Style {
            size: Size::new(Val::Px(16.0), Val::Px(16.0)),
            ..default()
        },
        ..default()
    }
}

fn content_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::Background.into(),
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Auto),
            flex_grow: 1.0,
            ..default()
        },
        ..Default::default()
    }
}
