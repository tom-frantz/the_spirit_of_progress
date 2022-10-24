use bevy::{prelude::*, render::texture::ImageSettings, ui::UiPlugin as BevyUiPlugin};
use the_spirit_of_progress::{
    camera::CameraPlugin,
    game::world::HexWorld,
    render::RenderPlugin,
    ui::{
        theme::{Colour, MenuColour},
        UiPlugin,
    },
    SpiritOfProgressPlugin,
};

fn main() {
    App::new()
        .add_plugin(SpiritOfProgressPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(HexWorld::new());
}
