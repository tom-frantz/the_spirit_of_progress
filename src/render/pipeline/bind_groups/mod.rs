use crate::render::prepare::MeshUniform;
use bevy::render::render_resource::DynamicUniformBuffer;

pub mod transform;
pub mod view;

pub type MeshUniformBuffer = DynamicUniformBuffer<MeshUniform>;
