use crate::ui::primitives::center_box::render_center_box;
use crate::ui::screens::Screen;
use bevy::prelude::*;

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

impl Screen for WeaponDesignScreen {
    fn draw(
        parent: &mut ChildBuilder,
        asset_server: &AssetServer,
        entity: Entity,
        component: &Self,
    ) {
        println!("DRAWING!");
        render_center_box(parent, |_| {});
    }
}
