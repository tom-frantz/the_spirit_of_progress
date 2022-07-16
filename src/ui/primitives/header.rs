use crate::ui::theme::TypographyColour;
use crate::ui::theme::SPACING;
use crate::ui::{clear_ui_elements, MainElements};
use bevy::prelude::Val::*;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum HeaderButton {
    Expand,
    Close,
}

impl HeaderButton {
    pub fn clicked(
        &self,
        mut commands: &mut Commands,
        q_ui_main_elements: &Query<Entity, With<MainElements>>,
    ) {
        match self {
            HeaderButton::Expand => {}
            HeaderButton::Close => clear_ui_elements(commands, q_ui_main_elements),
        }
    }
}

const BUTTON_SIZE: f32 = 16.0;

pub fn on_header_button_click(
    interaction_query: Query<(&Interaction, &HeaderButton), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    q_ui_main_elements: Query<Entity, With<MainElements>>,
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
                .spawn_bundle(button_bundle(TypographyColour::Yellow))
                .insert(HeaderButton::Expand);

            header
                .spawn_bundle(button_bundle(TypographyColour::Red))
                .insert(HeaderButton::Close);
        });
}

pub fn header_bundle() -> NodeBundle {
    NodeBundle {
        color: TypographyColour::Background.into(),
        style: Style {
            size: Size::new(Percent(100.0), Px(BUTTON_SIZE + SPACING)),
            justify_content: JustifyContent::FlexEnd,
            padding: Rect::all(Px(SPACING / 2.0)),
            margin: Rect {
                bottom: Px(SPACING),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}
pub fn button_bundle(color: TypographyColour) -> ButtonBundle {
    ButtonBundle {
        color: color.into(),
        style: Style {
            margin: Rect {
                left: Px(SPACING),
                ..default()
            },
            size: Size::new(Px(BUTTON_SIZE), Px(BUTTON_SIZE)),
            ..default()
        },
        ..default()
    }
}
