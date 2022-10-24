use crate::{
    camera::CameraPlugin,
    render::RenderPlugin,
    ui::{
        theme::{Colour, MenuColour},
        UiPlugin,
    },
};
use bevy::{prelude::*, render::texture::ImageSettings, ui::UiPlugin as BevyUiPlugin};

pub mod game;

pub mod camera;
pub mod render;
pub mod ui;

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
        // Spirit of Progress Plugin
        //
        app.add_plugin(UiPlugin)
            .add_plugin(RenderPlugin)
            .add_plugin(CameraPlugin);
    }
}
