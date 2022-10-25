use crate::game::weapons::small_arms::{SmallArmsComponents, SmallArmsComponentsAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

mod component;
mod small_arms;

pub struct WeaponsPlugin;
impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TomlAssetPlugin::<SmallArmsComponents>::new(&["toml"]))
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .with_collection::<SmallArmsComponentsAssets>(),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Loaded).with_system(print_loaded_components),
            );
    }
}

fn print_loaded_components(
    small_arms_res: Res<SmallArmsComponentsAssets>,
    assets: Res<Assets<SmallArmsComponents>>,
) {
    let small_arms = assets.get(&small_arms_res.components).unwrap();

    println!("SMALL ARMS: {small_arms:#?}");
}
