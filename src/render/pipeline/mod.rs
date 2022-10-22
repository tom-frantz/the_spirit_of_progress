use crate::render::pipeline::bind_groups::{
    transform::HexWorldTransformBindGroup, view::HexWorldViewBindGroup,
};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{
            BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendComponent, BlendFactor,
            BlendOperation, BlendState, BufferBindingType, ColorTargetState, ColorWrites, Face,
            FragmentState, FrontFace, MultisampleState, PolygonMode, PrimitiveState,
            PrimitiveTopology, RenderPipelineDescriptor, ShaderStages, ShaderType,
            SpecializedRenderPipeline, TextureFormat, VertexBufferLayout, VertexFormat,
            VertexState, VertexStepMode,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
        view::ViewUniform,
    },
};

pub const ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7896438451987165848);
pub const ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 1321874116519816237);

pub mod bind_groups;

#[derive(Clone)]
pub struct OrthographicHexagonPipeline {
    // The bind group layout for the view (window) information.
    pub view_layout: BindGroupLayout,
    pub transform_layout: BindGroupLayout,
}

impl FromWorld for OrthographicHexagonPipeline {
    fn from_world(world: &mut World) -> Self {
        debug!("CREATING PIPELINE FROM WORLD!");
        let world = world.cell();
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let view_layout = HexWorldViewBindGroup::create_bind_group_layout(&render_device);
        let transform_layout = HexWorldTransformBindGroup::create_bind_group_layout(&render_device);

        OrthographicHexagonPipeline {
            view_layout,
            transform_layout,
        }
    }
}

impl SpecializedRenderPipeline for OrthographicHexagonPipeline {
    type Key = ();

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let formats = vec![
            // Position
            VertexFormat::Float32x2,
            // Color
            VertexFormat::Float32x4,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vs_main".into(),
                shader_defs: vec![],
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                shader: ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE.typed::<Shader>(),
                shader_defs: vec![],
                entry_point: "fs_main".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            layout: Some(vec![
                self.view_layout.clone(),
                self.transform_layout.clone(),
            ]),
            primitive: PrimitiveState {
                conservative: false,
                cull_mode: Some(Face::Back),
                front_face: FrontFace::Ccw,
                polygon_mode: PolygonMode::Fill,
                strip_index_format: None,
                topology: PrimitiveTopology::TriangleList,
                unclipped_depth: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("orthographic_hexagon_pipeline".into()),
        }
    }
}
