use crate::ui::{
    fonts::Typography,
    primitives::center_box::{render_center_box, CenterBoxProps},
    screens::Screen,
    theme::{Colour, IndustryColour, MenuColour},
    utils::style_builder::StyleBuilder,
};
use bevy::prelude::{Val::*, *};

const HALF_BORDER: Val = Px(4.);

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
                let text = {
                    let mut text =
                        Typography::Title.text_section("-- Weapon Design --", asset_server);
                    text.sections[0].style.color = MenuColour::Background.color();
                    text
                };

                container.spawn_bundle(TextBundle {
                    style: StyleBuilder::new().align_self(AlignSelf::Center).build(),
                    text,
                    ..default()
                });
                render_center_box(
                    CenterBoxProps { bare: true },
                    container,
                    |center_box: &mut ChildBuilder| {
                        // The main content box
                        center_box
                            .spawn_bundle(
                                StyleBuilder::new()
                                    .flex_grow(true)
                                    .size(Percent(100.), Auto)
                                    .row()
                                    .build_clear_node_bundle(),
                            )
                            .with_children(|main_content| {
                                // Options Sidebar
                                main_content.spawn_bundle(Self::weapon_options_sidebar_bundle());
                                // Other content
                                main_content
                                    .spawn_bundle(
                                        StyleBuilder::new()
                                            .margin_left(HALF_BORDER)
                                            .size(Percent(100.), Percent(100.))
                                            .column()
                                            .build_clear_node_bundle(),
                                    )
                                    .with_children(|right_content| {
                                        // Gun Window + Stats
                                        right_content
                                            .spawn_bundle(
                                                StyleBuilder::new()
                                                    .margin_bottom(HALF_BORDER)
                                                    .size(Percent(100.), Percent(75.))
                                                    .row()
                                                    .build_clear_node_bundle(),
                                            )
                                            .with_children(|top_right_content| {
                                                // Gun Window
                                                top_right_content
                                                    .spawn_bundle(Self::weapon_window_bundle());
                                                // Stats sidebar
                                                top_right_content
                                                    .spawn_bundle(Self::stats_sidebar_bundle());
                                            });

                                        // Bottom Cost Panel
                                        right_content.spawn_bundle(Self::costs_panel_bundle());
                                    });
                            });
                    },
                );
            });
    }
}

impl WeaponDesignScreen {
    fn weapon_options_sidebar_bundle() -> NodeBundle {
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

    fn weapon_window_bundle() -> NodeBundle {
        NodeBundle {
            color: IndustryColour::Purple.ui_color(),
            style: StyleBuilder::new()
                .margin_right(HALF_BORDER)
                .size(Percent(75.), Percent(100.))
                .column()
                .build(),

            ..default()
        }
    }

    fn stats_sidebar_bundle() -> NodeBundle {
        NodeBundle {
            color: MenuColour::Background.ui_color(),
            style: StyleBuilder::new()
                .margin_left(HALF_BORDER)
                .size(Percent(25.), Percent(100.))
                .column()
                .build(),

            ..default()
        }
    }

    fn costs_panel_bundle() -> NodeBundle {
        NodeBundle {
            color: MenuColour::Background.ui_color(),
            style: StyleBuilder::new()
                .margin_top(HALF_BORDER)
                .size(Percent(100.), Percent(25.))
                .build(),

            ..default()
        }
    }
}
