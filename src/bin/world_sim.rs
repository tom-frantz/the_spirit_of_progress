use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        texture::ImageSettings,
    },
    ui::UiPlugin,
};
use bevy_ecs_tilemap::TilemapPlugin;
use the_spirit_of_progress::SpiritOfProgressPlugin;

fn main() {
    App::new().add_plugin(SpiritOfProgressPlugin).run()
}
