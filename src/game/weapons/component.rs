use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Debug)]
pub struct WeaponComponent<Stats>
where
    Stats: Debug,
{
    name: String,
    asset: WeaponAsset,

    statistics: Stats,
    costs: WeaponProductionCost,
}

#[derive(Deserialize, Debug)]
pub struct WeaponProductionCost {
    #[serde(default)]
    wood: f32,
    #[serde(default)]
    metal: f32,
}

#[derive(Deserialize, Debug)]
struct WeaponAsset {
    filename: String,

    #[serde(default)]
    offset: AssetOffset,
}

#[derive(Deserialize, Debug, Default)]
struct AssetOffset {
    x: f32,
    y: f32,
}
