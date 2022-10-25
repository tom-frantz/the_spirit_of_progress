use crate::{
    camera::CameraPlugin,
    game::weapons::WeaponsPlugin,
    render::RenderPlugin,
    ui::{
        theme::{Colour, MenuColour},
        UiPlugin,
    },
};
use bevy::{prelude::*, render::texture::ImageSettings, ui::UiPlugin as BevyUiPlugin};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub mod game;

pub mod camera;
pub mod render;
pub mod ui;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Loaded,
}

pub struct SpiritOfProgressPlugin;
impl Plugin for SpiritOfProgressPlugin {
    fn build(&self, app: &mut App) {
        //
        // Bevy Inbuilts
        //
        app.insert_resource(WindowDescriptor {
            title: "The Spirit of Progress".to_string(),
            ..default()
        })
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(MenuColour::BlackPen.color()))
        // .insert_resource(WgpuSettings {
        //     backends: Some(bevy::render::settings::Backends::DX12),
        //     ..Default::default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyUiPlugin)
        .insert_resource(ImageSettings::default_nearest());

        //
        // 3rd Party Plugins
        //

        //
        // Spirit of Progress Plugin
        //
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Loaded),
        )
        .add_state(GameState::AssetLoading)
        .add_plugin(UiPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WeaponsPlugin);
    }
}
