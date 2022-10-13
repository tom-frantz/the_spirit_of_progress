use self::{
    pipeline::{
        OrthographicHexagonPipeline, ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE,
        ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE,
    },
    shader::include_ortho_hex_shader,
};
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::system::lifetimeless::{Read, SQuery, SRes},
    ecs::system::SystemParamItem,
    prelude::*,
    render::{
        mesh::PrimitiveTopology,
        mesh::{GpuBufferInfo, GpuMesh, Indices, MeshVertexAttribute, VertexAttributeValues},
        render_phase::{
            AddRenderCommand, DrawFunctions, RenderCommand, RenderCommandResult, RenderPhase,
            TrackedRenderPass,
        },
        render_resource::VertexFormat,
        render_resource::{
            BufferInitDescriptor, BufferUsages, PipelineCache, SpecializedRenderPipelines,
        },
        renderer::RenderDevice,
        view::ExtractedView,
        RenderApp, RenderStage,
    },
    utils::FloatOrd,
};
use extract::extract;
use prepare::prepare;
use queue::queue;

mod pipeline;
mod shader;
mod utils;

mod extract;
mod prepare;
mod queue;

pub mod traits;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        let ortho_shader = include_ortho_hex_shader();

        shaders.set_untracked(
            ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE,
            ortho_shader.vertex,
        );
        shaders.set_untracked(
            ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE,
            ortho_shader.fragment,
        );

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_system_to_stage(RenderStage::Extract, extract)
            .add_system_to_stage(RenderStage::Prepare, prepare)
            .add_system_to_stage(RenderStage::Queue, queue)
            .init_resource::<OrthographicHexagonPipeline>()
            .init_resource::<SpecializedRenderPipelines<OrthographicHexagonPipeline>>();

        render_app.add_render_command::<Transparent2d, DrawHexWorld>();
    }
}

type LatLngVertex = [f32; 2];

// TODO make this actually useful.
struct HexWorld(pub u8, GpuMesh);

#[derive(Component, Debug)]
pub struct HexWorldId(u8);
impl HexWorld {
    pub fn new(id: u8, gpu_mesh: GpuMesh) -> Self {
        HexWorld(id, gpu_mesh)
    }
}

struct DrawHexWorld;
impl RenderCommand<Transparent2d> for DrawHexWorld {
    type Param = (
        SRes<PipelineCache>,
        SRes<HexWorld>,
        SQuery<(Read<HexWorldId>)>,
    );

    fn render<'w>(
        _view: Entity,
        item: &Transparent2d,
        (pipeline_cache, hex_world_res, hex_world_query): SystemParamItem<'w, '_, Self::Param>,
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

        let (_hex_world_id) = hex_world_query.get(item.entity).unwrap();

        // Get the gpu mesh from the prepare stage.
        let gpu_mesh: &GpuMesh = &hex_world_res.into_inner().1;

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

        RenderCommandResult::Success
    }
}
