use crate::render::pipeline::bind_groups::view::HexWorldViewBindGroup;
use crate::render::{HexWorld, HexWorldChunk};
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    prelude::*,
    render::{
        mesh::{GpuBufferInfo, GpuMesh},
        render_phase::{RenderCommand, RenderCommandResult, TrackedRenderPass},
        render_resource::PipelineCache,
        view::ViewUniformOffset,
    },
};

pub struct SetMeshViewBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetMeshViewBindGroup<I> {
    type Param = SQuery<(Read<ViewUniformOffset>, Read<HexWorldViewBindGroup>)>;
    #[inline]
    fn render<'w>(
        view: Entity,
        _item: &Transparent2d,
        view_query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // IDK what pbr means, I just nicked this from bevy_ecs_tilemap. check there for more info?
        let (view_uniform, pbr_view_bind_group) = view_query.get_inner(view).unwrap();
        pass.set_bind_group(I, &pbr_view_bind_group.value, &[view_uniform.offset]);

        RenderCommandResult::Success
    }
}

pub struct SetTransformBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent2d> for SetTransformBindGroup<I> {
    type Param = SQuery<(Read<Transform>)>;
    #[inline]
    fn render<'w>(
        view: Entity,
        item: &Transparent2d,
        view_query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // pass.set_bind_group(I, &, &[]);

        RenderCommandResult::Success
    }
}

pub type DrawHexWorld = (SetMeshViewBindGroup<0>, SetTransformBindGroup<1>, DrawMesh);

pub struct DrawMesh;
impl RenderCommand<Transparent2d> for DrawMesh {
    type Param = (SRes<PipelineCache>, SQuery<(Read<HexWorldChunk>)>);

    fn render<'w>(
        _view: Entity,
        item: &Transparent2d,
        (pipeline_cache, hex_world_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        if let Some(pipeline) = pipeline_cache
            .into_inner()
            .get_render_pipeline(item.pipeline)
        {
            pass.set_render_pipeline(pipeline);
        } else {
            return RenderCommandResult::Failure;
        }

        for (hex_world) in hex_world_query.iter_inner() {
            // Get the gpu mesh from the prepare stage.
            let gpu_mesh: &GpuMesh = &hex_world.1;

            // Set up the render pass with the data from the GPU mesh.
            pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            match &gpu_mesh.buffer_info {
                GpuBufferInfo::Indexed {
                    buffer,
                    index_format,
                    count,
                } => {
                    pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                    pass.draw_indexed(0..*count, 0, 0..1);
                }
                GpuBufferInfo::NonIndexed { vertex_count } => {
                    pass.draw(0..*vertex_count, 0..1);
                }
            }
        }
        RenderCommandResult::Success
    }
}
