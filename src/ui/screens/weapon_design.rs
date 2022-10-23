use crate::ui::{
    fonts::Typography,
    primitives::center_box::{render_center_box, CenterBoxProps},
    screens::Screen,
    theme::{Colour, IndustryColour, MenuColour},
    utils::style_builder::StyleBuilder,
};
use bevy::prelude::{Val::*, *};

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

pub fn flex_row_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

impl Screen for WeaponDesignScreen {
    fn draw(
        parent: &mut ChildBuilder,
        asset_server: &AssetServer,
        _entity: Entity,
        _component: &Self,
    ) {
        parent
            .spawn_bundle(NodeBundle {
                color: MenuColour::BlackPen.ui_color(),
                style: StyleBuilder::new()
                    .column()
                    .size(Percent(100.), Percent(100.))
                    .padding(Px(48.0))
                    .justify_content(JustifyContent::Center)
                    .build(),
                ..default()
            })
            .with_children(|container| {
                let mut text = Typography::Title.text_section("-- Weapon Design --", asset_server);
                text.sections[0].style.color = MenuColour::Background.color();

                container.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: text,
                    ..default()
                });
                render_center_box(
                    CenterBoxProps { bare: true },
                    container,
                    |center_box: &mut ChildBuilder| {
                        // The main content box
                        center_box
                            .spawn_bundle(NodeBundle {
                                color: Color::NONE.into(),
                                style: StyleBuilder::new()
                                    .flex_grow(true)
                                    .size(Percent(100.), Auto)
                                    .row()
                                    .build(),

                                ..default()
                            })
                            .with_children(|main_content| {
                                // Options Sidebar
                                let half_border = Px(4.);
                                main_content.spawn_bundle(NodeBundle {
                                    color: MenuColour::Background.ui_color(),
                                    style: StyleBuilder::new()
                                        .margin_right(half_border)
                                        .size(Percent(25.), Auto)
                                        .column()
                                        .build(),

                                    ..default()
                                });
                                // Other content
                                main_content
                                    .spawn_bundle(NodeBundle {
                                        color: Color::NONE.into(),
                                        style: StyleBuilder::new()
                                            .margin_left(half_border)
                                            .size(Percent(100.), Percent(100.))
                                            .column()
                                            .build(),

                                        ..default()
                                    })
                                    .with_children(|right_content| {
                                        // Gun Window + Stats
                                        right_content
                                            .spawn_bundle(NodeBundle {
                                                color: Color::NONE.into(),
                                                style: StyleBuilder::new()
                                                    .margin_bottom(half_border)
                                                    .size(Percent(100.), Percent(75.))
                                                    .row()
                                                    .build(),

                                                ..default()
                                            })
                                            .with_children(|top_right_content| {
                                                // Gun Window
                                                top_right_content.spawn_bundle(NodeBundle {
                                                    color: IndustryColour::Purple.ui_color(),
                                                    style: StyleBuilder::new()
                                                        .margin_right(half_border)
                                                        .size(Percent(75.), Percent(100.))
                                                        .column()
                                                        .build(),

                                                    ..default()
                                                });
                                                // Stats sidebar
                                                top_right_content.spawn_bundle(NodeBundle {
                                                    color: MenuColour::Background.ui_color(),
                                                    style: StyleBuilder::new()
                                                        .margin_left(half_border)
                                                        .size(Percent(25.), Percent(100.))
                                                        .column()
                                                        .build(),

                                                    ..default()
                                                });
                                            });

                                        // Bottom Cost Panel
                                        right_content.spawn_bundle(NodeBundle {
                                            color: MenuColour::Background.ui_color(),
                                            style: StyleBuilder::new()
                                                .margin_top(half_border)
                                                .size(Percent(100.), Percent(25.))
                                                .build(),

                                            ..default()
                                        });
                                    });
                            });
                    },
                );
            });
    }
}
