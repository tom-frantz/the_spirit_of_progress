use crate::render::prepare::MeshUniform;
use bevy::{
    prelude::*,
    render::{
        render_resource::{BindGroup, BindGroupLayout, BindingResource, DynamicUniformBuffer},
        renderer::RenderDevice,
    },
};
use std::marker::PhantomData;

pub mod transform;
pub mod view;

pub type MeshUniformBuffer = DynamicUniformBuffer<MeshUniform>;
