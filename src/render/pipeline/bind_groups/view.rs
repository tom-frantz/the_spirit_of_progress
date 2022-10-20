use bevy::render::render_resource::{BindGroupDescriptor, BindGroupEntry, BindingResource};
use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, BufferBindingType, ShaderStages, ShaderType,
        },
        renderer::RenderDevice,
        view::ViewUniform,
    },
};

/// HexWorldViewBindGroup
/// This bind group describes the size/projection/etc. of the current view, or window, that is
/// being drawn to at that time.
#[derive(Component)]
pub struct HexWorldViewBindGroup {
    pub value: BindGroup,
}
impl HexWorldViewBindGroup {
    pub(crate) fn new(render_device: &RenderDevice, view_binding: &BindingResource) -> Self {
        HexWorldViewBindGroup {
            value: Self::create_bind_group(render_device, view_binding),
        }
    }

    fn create_bind_group(
        render_device: &RenderDevice,
        view_binding: &BindingResource,
    ) -> BindGroup {
        render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_binding.clone(),
            }],
            // TODO pull this from &orthographic_pipeline.view_layout instead.
            layout: &*Self::create_bind_group_layout(render_device),
            label: Some("hexworld_view_bind_group"),
        })
    }

    pub fn create_bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    // TODO: change this to ViewUniform::std140_size_static once crevice fixes this!
                    // Context: https://github.com/LPGhatguy/crevice/issues/29
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            }],
            label: Some("hexworld_view_bind_group_layout"),
        })
    }
}
