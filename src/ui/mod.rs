use crate::Component;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub mod components;
pub mod fonts;
pub mod interaction;
mod utils;

#[derive(Bundle)]
pub struct LabelledNodeBundle<Label: Component> {
    #[bundle]
    node_bundle: NodeBundle,
    label: Label,
}

#[derive(Bundle)]
pub struct LabelledTextBundle<Label: Component> {
    #[bundle]
    node_bundle: TextBundle,
    label: Label,
}
