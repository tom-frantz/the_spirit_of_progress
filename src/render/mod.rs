use self::{
    pipeline::{
        OrthographicHexagonPipeline, ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE,
        ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE,
    },
    shader::include_ortho_hex_shader,
};
use crate::render::prepare::{DynamicUniformIndex, MeshUniform};
use bevy::render::render_resource::DynamicUniformBuffer;
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::system::{
        lifetimeless::{Read, SQuery, SRes},
        SystemParamItem,
    },
    prelude::*,
    render::{
        mesh::{
            GpuBufferInfo, GpuMesh, Indices, MeshVertexAttribute, PrimitiveTopology,
            VertexAttributeValues,
        },
        render_phase::{
            AddRenderCommand, DrawFunctions, RenderCommand, RenderCommandResult, RenderPhase,
            TrackedRenderPass,
        },
        render_resource::{
            BufferInitDescriptor, BufferUsages, PipelineCache, SpecializedRenderPipelines,
            VertexFormat,
        },
        renderer::RenderDevice,
        view::ExtractedView,
        RenderApp, RenderStage,
    },
    utils::FloatOrd,
};
use draw::DrawHexWorld;
use extract::extract;
use prepare::prepare;
use queue::queue;

mod draw;
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
            .init_resource::<SpecializedRenderPipelines<OrthographicHexagonPipeline>>()
            .init_resource::<DynamicUniformBuffer<MeshUniform>>();

        render_app.add_render_command::<Transparent2d, DrawHexWorld>();
    }
}

type LatLngVertex = [f32; 2];

// TODO make this actually useful.
pub struct HexWorld(pub u8, GpuMesh);

#[derive(Component, Debug)]
pub struct HexWorldChunk(pub u8, GpuMesh);
impl HexWorld {
    pub fn new(id: u8, gpu_mesh: GpuMesh) -> Self {
        HexWorld(id, gpu_mesh)
    }
}
