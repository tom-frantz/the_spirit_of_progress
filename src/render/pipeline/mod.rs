use bevy::render::render_resource::{
    RenderPipelineDescriptor, SpecializedRenderPipeline, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};

pub struct OrthographicHexagonPipeline {}

impl SpecializedRenderPipeline for OrthographicHexagonPipeline {
    type Key = ();

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let shader = "ORTHOGRAPHIC_HEXAGON_SHADER";

        let formats = vec![
            // Position
            VertexFormat::Float32x4,
            // Uv
            VertexFormat::Float32x4,
            // Color
            VertexFormat::Float32x4,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: TILEMAP_SHADER_VERTEX.typed::<Shader>(),
                entry_point: "vertex_main".into(),
                shader_defs: shader.clone().into(),
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                shader: TILEMAP_SHADER_FRAGMENT.typed::<Shader>(),
                shader_defs,
                entry_point: "fragment".into(),
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
                self.mesh_layout.clone(),
                self.uniform_layout.clone(),
                self.material_layout.clone(),
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
                count: key.msaa,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("tilemap_pipeline".into()),
        }
    }
}
