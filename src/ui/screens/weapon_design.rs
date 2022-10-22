use crate::ui::{
    fonts::Typography,
    primitives::center_box::render_center_box,
    screens::Screen,
    theme::{Colour, IndustryColour},
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum WeaponDesignMode {
    Gun,
}

impl Default for WeaponDesignMode {
    fn default() -> Self {
        WeaponDesignMode::Gun
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct WeaponDesignScreen {
    weapon_mode: WeaponDesignMode,
}

impl Screen for WeaponDesignScreen {
    fn draw(
        parent: &mut ChildBuilder,
        asset_server: &AssetServer,
        _entity: Entity,
        _component: &Self,
    ) {
        render_center_box(parent, |center_box| {
            center_box
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_grow: 0.,
                        ..default()
                    },
                    color: IndustryColour::LightRed.ui_color(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style { ..default() },
                        text: Typography::Title.with_section("Weapon Design", asset_server),
                        ..default()
                    });
                });

            center_box.spawn_bundle(TextBundle {
                text: Typography::Subtitle.with_section("Weapon Design 2", asset_server),
                ..default()
            });
            center_box.spawn_bundle(TextBundle {
                text: Typography::Body.with_section("Weapon Design 2", asset_server),
                ..default()
            });

            center_box.spawn_bundle(TextBundle {
                text: Typography::BodyBold.with_section("Weapon Design 2", asset_server),
                ..default()
            });
        });
    }
}
