use bevy::{prelude::*, render::texture::ImageSettings, ui::UiPlugin as BevyUiPlugin};
use the_spirit_of_progress::{
    camera::CameraPlugin,
    game::world::HexWorld,
    render::RenderPlugin,
    ui::{
        theme::{Colour, MenuColour},
        UiPlugin,
    },
};

fn main() {
    App::new()
        // Bevy
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(MenuColour::BlackPen.color()))
        // .insert_resource(WgpuSettings {
        //     backends: Some(bevy::render::settings::Backends::DX12),
        //     ..Default::default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyUiPlugin)
        // Spirit of Progress
        .add_plugin(UiPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(CameraPlugin)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(HexWorld::new());
}
