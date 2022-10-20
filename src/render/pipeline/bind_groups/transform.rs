use crate::render::prepare::MeshUniform;
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferBindingType, DynamicUniformBuffer, ShaderStages, ShaderType,
};
use bevy::render::view::ViewUniform;
use bevy::{
    prelude::*,
    render::{
        render_resource::{BindGroup, BindGroupLayout, BindingResource},
        renderer::RenderDevice,
    },
};

pub struct HexWorldTransformBindGroup {
    pub value: BindGroup,
}

impl HexWorldTransformBindGroup {
    pub fn new(
        render_device: &RenderDevice,
        transform_uniform_buffer_binding: BindingResource,
    ) -> Self {
        HexWorldTransformBindGroup {
            value: Self::create_bind_group(render_device, transform_uniform_buffer_binding),
        }
    }

    fn create_bind_group(
        render_device: &RenderDevice,
        transform_uniform_buffer_binding: BindingResource,
    ) -> BindGroup {
        render_device.create_bind_group(&BindGroupDescriptor {
            layout: &*Self::create_bind_group_layout(render_device),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_uniform_buffer_binding,
            }],
            label: Some("hexworld_transform_bind_group"),
        })
    }

    // fn get_bind_group_index

    fn create_bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    // TODO: change this to ViewUniform::std140_size_static once crevice fixes this!
                    // Context: https://github.com/LPGhatguy/crevice/issues/29
                    min_binding_size: Some(MeshUniform::min_size()),
                },
                count: None,
            }],
            label: Some("hexworld_transform_bind_group_layout"),
        })
    }
}
