use crate::ui::LabelledNodeBundle;
use bevy::prelude::*;

pub mod city_info;

#[derive(Component)]
pub struct RootNode;
fn root_node() -> LabelledNodeBundle<RootNode> {
    LabelledNodeBundle {
        node_bundle: NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        },
        label: RootNode,
    }
}

pub fn render_root_ui(mut commands: Commands) {
    commands.spawn_bundle(root_node());
}
