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

mod pipeline;
mod shader;
mod utils;

mod extract;
mod prepare;
mod queue;

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
            .add_system_to_stage(RenderStage::Extract, extract::extract)
            .add_system_to_stage(RenderStage::Prepare, prepare::prepare)
            .add_system_to_stage(RenderStage::Queue, queue::queue)
            .init_resource::<OrthographicHexagonPipeline>()
            .init_resource::<SpecializedRenderPipelines<OrthographicHexagonPipeline>>();

        render_app.add_render_command::<Transparent2d, DrawHexWorld>();
    }
}

type LatLngVertex = [f32; 2];

#[derive(Component, Debug, Clone, Copy)]
struct Hexagon {
    vertices: [LatLngVertex; 6],
}

impl Hexagon {
    pub fn from_center(position: [f32; 2]) -> Self {
        let radius: f32 = 0.2;
        let cos_60: f32 = 60_f32.to_radians().cos();
        let sin_60: f32 = 60_f32.to_radians().sin();

        Hexagon {
            vertices: [
                [position[0] + radius * cos_60, position[1] + radius * sin_60],
                [position[0] - radius * cos_60, position[1] + radius * sin_60],
                [position[0] - radius, position[1]],
                [position[0] - radius * cos_60, position[1] - radius * sin_60],
                [position[0] + radius * cos_60, position[1] - radius * sin_60],
                [position[0] + radius, position[1]],
            ],
        }
    }

    pub fn vertices(&self) -> [LatLngVertex; 6] {
        self.vertices
    }

    pub fn indices(&self, adjustment: u16) -> [[u16; 3]; 4] {
        [
            [0 + adjustment, 1 + adjustment, 2 + adjustment],
            [0 + adjustment, 2 + adjustment, 3 + adjustment],
            [0 + adjustment, 3 + adjustment, 4 + adjustment],
            [0 + adjustment, 4 + adjustment, 5 + adjustment],
        ]
    }
}

// TODO make this actually useful.
struct HexWorld(pub u8, GpuMesh);

#[derive(Component, Debug)]
struct HexWorldId(u8);
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
        // println!("DRAWING!");

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
        // println!("{:?}", gpu_mesh);
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
                panic!("wowza");
                pass.draw(0..*vertex_count, 0..1);
            }
        }

        RenderCommandResult::Success
    }
}
