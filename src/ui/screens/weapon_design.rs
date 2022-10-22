use crate::ui::{
    fonts::Typography,
    primitives::center_box::render_center_box,
    primitives::center_box::CenterBoxProps,
    screens::Screen,
    theme::MenuColour,
    theme::{Colour, IndustryColour},
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

const BORDER_SIZE: f32 = 4.;

pub fn flex_column_style() -> Style {
    Style {
        flex_direction: FlexDirection::ColumnReverse,
        ..default()
    }
}

pub fn flex_row_style() -> Style {
    Style {
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

pub fn full_size_ui_box() -> NodeBundle {
    NodeBundle {
        color: MenuColour::Background.ui_color(),
        style: Style {
            size: Size::new(Percent(100.), Percent(100.)),
            ..default()
        },
        ..default()
    }
}

pub fn full_size_gun_box() -> NodeBundle {
    NodeBundle {
        color: MenuColour::Border.ui_color(),
        style: Style {
            size: Size::new(Percent(100.), Percent(100.)),
            ..default()
        },
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
        render_center_box(
            CenterBoxProps { bare: true },
            parent,
            |center_box: &mut ChildBuilder| {
                // Title
                center_box.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Typography::Title.text_section("Weapon Design", asset_server),
                    ..default()
                });

                // The main content box
                center_box
                    .spawn_bundle(NodeBundle {
                        color: Color::NONE.into(),
                        style: Style {
                            flex_grow: 1.,
                            size: Size::new(Percent(100.), Auto),
                            ..flex_row_style()
                        },
                        ..default()
                    })
                    .with_children(|main_content| {
                        // Options Sidebar
                        let half_border = Px(2.);
                        main_content
                            .spawn_bundle(NodeBundle {
                                color: IndustryColour::Purple.ui_color(),
                                // color: Color::NONE.into(),
                                style: Style {
                                    margin: UiRect::new(Px(0.), half_border, Px(0.), Px(0.)),
                                    padding: UiRect::all(Px(BORDER_SIZE)),
                                    size: Size::new(Percent(25.), Auto),
                                    ..flex_column_style()
                                },
                                ..default()
                            })
                            .with_children(|p| {
                                p.spawn_bundle(full_size_ui_box());
                            });
                        // Other content
                        main_content
                            .spawn_bundle(NodeBundle {
                                color: Color::NONE.into(),

                                style: Style {
                                    margin: UiRect::new(half_border, Px(0.), Px(0.), Px(0.)),

                                    size: Size::new(Percent(100.), Percent(100.)),
                                    ..flex_column_style()
                                },
                                ..default()
                            })
                            .with_children(|right_content| {
                                // Gun Window + Stats
                                right_content
                                    .spawn_bundle(NodeBundle {
                                        color: Color::NONE.into(),

                                        style: Style {
                                            margin: UiRect::new(
                                                Px(0.),
                                                Px(0.),
                                                Px(0.),
                                                half_border,
                                            ),

                                            size: Size::new(Percent(100.), Percent(75.)),
                                            ..flex_row_style()
                                        },
                                        ..default()
                                    })
                                    .with_children(|top_right_content| {
                                        // Gun Window
                                        top_right_content
                                            .spawn_bundle(NodeBundle {
                                                // color: MenuColour::BorderBackground.ui_color(),
                                                color: IndustryColour::Purple.ui_color(),

                                                style: Style {
                                                    margin: UiRect::new(
                                                        Px(0.),
                                                        half_border,
                                                        Px(0.),
                                                        Px(0.),
                                                    ),
                                                    padding: UiRect::all(Px(BORDER_SIZE)),

                                                    size: Size::new(Percent(75.), Percent(100.)),
                                                    ..flex_column_style()
                                                },
                                                ..default()
                                            })
                                            .with_children(|p| {
                                                p.spawn_bundle(full_size_gun_box());
                                            });
                                        // Stats sidebar
                                        top_right_content
                                            .spawn_bundle(NodeBundle {
                                                // color: MenuColour::Background.ui_color(),
                                                color: IndustryColour::Purple.ui_color(),

                                                style: Style {
                                                    margin: UiRect::new(
                                                        half_border,
                                                        Px(0.),
                                                        Px(0.),
                                                        Px(0.),
                                                    ),
                                                    padding: UiRect::all(Px(BORDER_SIZE)),

                                                    size: Size::new(Percent(25.), Percent(100.)),
                                                    ..flex_column_style()
                                                },
                                                ..default()
                                            })
                                            .with_children(|p| {
                                                p.spawn_bundle(full_size_ui_box());
                                            });
                                    });

                                // Bottom Cost Panel
                                right_content
                                    .spawn_bundle(NodeBundle {
                                        // color: MenuColour::Background.ui_color(),
                                        color: IndustryColour::Purple.ui_color(),

                                        style: Style {
                                            margin: UiRect::new(
                                                Px(0.),
                                                Px(0.),
                                                half_border,
                                                Px(0.),
                                            ),
                                            padding: UiRect::all(Px(BORDER_SIZE)),

                                            size: Size::new(Percent(100.), Percent(25.)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|p| {
                                        p.spawn_bundle(full_size_ui_box());
                                    });
                            });
                    });
            },
        );
    }
}
