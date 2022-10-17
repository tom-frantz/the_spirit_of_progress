use crate::ui::theme::SPACING;
use crate::ui::theme::{Colour, MenuColour};
use crate::ui::utils::clear_ui_elements;
use crate::ui::RootElement;
use bevy::prelude::Val::*;
use bevy::prelude::*;
use bevy::ui::UiRect;

#[derive(Component, Debug)]
pub enum HeaderButton {
    Expand,
    Close,
}

impl HeaderButton {
    pub fn clicked(
        &self,
        commands: &mut Commands,
        q_ui_main_elements: &Query<Entity, With<RootElement>>,
    ) {
        match self {
            HeaderButton::Expand => {}
            HeaderButton::Close => clear_ui_elements(commands, q_ui_main_elements),
        }
    }
}

const BUTTON_SIZE: f32 = 16.0;

pub(super) fn on_header_button_click(
    interaction_query: Query<(&Interaction, &HeaderButton), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    q_ui_main_elements: Query<Entity, With<RootElement>>,
) {
    for (interaction, header_button) in &mut interaction_query.iter() {
        match interaction {
            Interaction::Clicked => header_button.clicked(&mut commands, &q_ui_main_elements),
            _ => {}
        };
    }
}

pub fn render_header(parent: &mut ChildBuilder) {
    parent
        .spawn_bundle(header_bundle())
        .with_children(|header| {
            header
                .spawn_bundle(button_bundle(MenuColour::Chart))
                .insert(HeaderButton::Expand);

            header
                .spawn_bundle(button_bundle(MenuColour::RedPen))
                .insert(HeaderButton::Close);
        });
}

pub fn header_bundle() -> NodeBundle {
    NodeBundle {
        color: UiColor(MenuColour::Background.color()),
        style: Style {
            size: Size::new(Percent(100.0), Px(BUTTON_SIZE + SPACING)),
            justify_content: JustifyContent::FlexEnd,
            padding: UiRect::all(Px(SPACING / 2.0)),
            margin: UiRect {
                bottom: Px(SPACING),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}
pub fn button_bundle(color: MenuColour) -> ButtonBundle {
    ButtonBundle {
        color: UiColor(color.color()),
        style: Style {
            margin: UiRect {
                left: Px(SPACING),
                ..default()
            },
            size: Size::new(Px(BUTTON_SIZE), Px(BUTTON_SIZE)),
            ..default()
        },
        ..default()
    }
}
