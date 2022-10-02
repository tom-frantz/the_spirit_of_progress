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

fn extract() {
    println!("Hey this should be happening?! EXTRACT");
}

pub const ATTRIBUTE_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Position", 184657321, VertexFormat::Float32x2);

pub const ATTRIBUTE_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("Color", 218479653, VertexFormat::Float32x4);

// TODO make this actually useful.
struct HexWorld(pub u8, GpuMesh);

#[derive(Component, Debug)]
struct HexWorldId(u8);
impl HexWorld {
    pub fn new(id: u8, gpu_mesh: GpuMesh) -> Self {
        HexWorld(id, gpu_mesh)
    }
}

fn prepare(mut commands: Commands, render_device: Res<RenderDevice>) {
    let mut tiles = Vec::new();

    // Store data somewhere before it's converted for the GPU
    let mut positions: Vec<[f32; 2]> = Vec::new();
    let mut colours: Vec<[f32; 4]> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    for index in 0..=1 {
        let hex = Hexagon::from_center([0.0 + index as f32 * 0.4, 0.0]);

        for vertex in &hex.vertices {
            positions.push(*vertex);
            // println!("{vertex:?}");
            colours.push([0.8, 0.8, 0.3, 1.0])
        }

        // the `+ 5` is tied to the (temporary) iterator that starts at `-5`
        for index in hex.indices((index) as u16 * 6) {
            indices.extend(index);
        }

        tiles.push(hex);
        // commands.spawn().insert(hex);
    }

    // Create a mesh to get data from, for the render device.
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // Insert data into the mesh, so that it can then be sent to the GPU
    mesh.insert_attribute(
        ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x2(positions),
    );
    mesh.insert_attribute(ATTRIBUTE_COLOR, VertexAttributeValues::Float32x4(colours));
    mesh.set_indices(Some(Indices::U16(indices)));

    // Create the buffers with data for the GPU
    let vertex_buffer_data = mesh.get_vertex_buffer_data();
    let vertex_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("hex_buffer"),
        contents: &vertex_buffer_data,
        usage: BufferUsages::VERTEX,
    });

    let buffer_info = mesh.get_index_buffer_bytes().map_or(
        GpuBufferInfo::NonIndexed {
            vertex_count: mesh.count_vertices() as u32,
        },
        |data| GpuBufferInfo::Indexed {
            buffer: render_device.create_buffer_with_data(&BufferInitDescriptor {
                usage: BufferUsages::INDEX,
                contents: data,
                label: Some("Mesh Index Buffer"),
            }),
            count: mesh.indices().unwrap().len() as u32,
            index_format: mesh.indices().unwrap().into(),
        },
    );

    // Finalize the shape of this for the GPU. This will be used in the draw function!
    let gpu_mesh = GpuMesh {
        vertex_buffer,
        buffer_info,
        primitive_topology: PrimitiveTopology::TriangleList,
        layout: mesh.get_mesh_vertex_buffer_layout(),
    };

    commands.insert_resource(HexWorld::new(0, gpu_mesh));
    commands.spawn().insert(HexWorldId(0));
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

fn queue(
    transparent_2d_draw_functions: Res<DrawFunctions<Transparent2d>>,

    ortho_hexagon_pipeline: Res<OrthographicHexagonPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<OrthographicHexagonPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,

    mut views: Query<(Entity, &ExtractedView, &mut RenderPhase<Transparent2d>)>,
    mut prepared_hexagons: Query<(Entity, &HexWorldId)>,
) {
    for (entity, _view, mut transparent_phase) in views.iter_mut() {
        let pipeline_id = pipelines.specialize(&mut pipeline_cache, &ortho_hexagon_pipeline, ());
        let draw_hexagon = transparent_2d_draw_functions
            .read()
            .get_id::<DrawHexWorld>()
            .unwrap();

        for (entity, _hex_world) in prepared_hexagons.iter() {
            transparent_phase.add(Transparent2d {
                entity,
                pipeline: pipeline_id,
                draw_function: draw_hexagon,
                batch_range: None,
                sort_key: FloatOrd(0.0),
            })
        }
    }
}
