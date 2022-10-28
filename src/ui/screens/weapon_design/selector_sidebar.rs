use crate::{
    ui::{
        screens::{weapon_design::HALF_BORDER, ChangedUiQuery},
        utils::style_builder::StyleBuilder,
        LabelledNodeBundle,
    },
    Colour, MenuColour,
};
use bevy::prelude::{Val::*, *};

#[derive(Component)]
pub struct SelectorSidebar;

impl SelectorSidebar {
    pub fn bundle() -> LabelledNodeBundle<SelectorSidebar> {
        LabelledNodeBundle {
            node_bundle: Self::content_bundle(),
            label: SelectorSidebar,
        }
    }

    pub fn on_change(mut commands: Commands, change_query: ChangedUiQuery<Self>) {}

    fn content_bundle() -> NodeBundle {
        NodeBundle {
            color: MenuColour::Background.ui_color(),
            style: StyleBuilder::new()
                .margin_right(HALF_BORDER)
                .size(Percent(25.), Auto)
                .column()
                .build(),

            ..default()
        }
    }
}
