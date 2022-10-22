use crate::render::{
    draw::DrawHexWorld,
    pipeline::{
        bind_groups::{transform::HexWorldTransformBindGroup, view::HexWorldViewBindGroup},
        OrthographicHexagonPipeline,
    },
    prepare::MeshUniform,
    HexWorldChunk,
};
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    prelude::{Commands, Entity, Query, Res, ResMut},
    render::{
        render_phase::{DrawFunctions, RenderPhase},
        render_resource::{DynamicUniformBuffer, PipelineCache, SpecializedRenderPipelines},
        renderer::RenderDevice,
        view::{ExtractedView, ViewUniforms},
    },
    utils::FloatOrd,
};

pub fn queue(
    mut commands: Commands,
    transparent_2d_draw_functions: Res<DrawFunctions<Transparent2d>>,

    ortho_hexagon_pipeline: Res<OrthographicHexagonPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<OrthographicHexagonPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,

    // The GPU device
    render_device: Res<RenderDevice>,

    // The windows of the game
    mut views: Query<(Entity, &ExtractedView, &mut RenderPhase<Transparent2d>)>,
    // The view ... uniforms? I don't know what this is but I think it's the screen?
    // (or probably mor accurately, each window)?
    view_uniforms: Res<ViewUniforms>,
    transform_uniforms: Res<DynamicUniformBuffer<MeshUniform>>,

    prepared_hexagons: Query<(Entity, &HexWorldChunk)>,
) {
    if let Some(binding) = transform_uniforms.binding() {
        commands.insert_resource(HexWorldTransformBindGroup::new(&render_device, binding));
    }

    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        for (entity, _view, mut transparent_phase) in views.iter_mut() {
            commands
                .entity(entity)
                .insert(HexWorldViewBindGroup::new(&render_device, &view_binding));

            let pipeline_id =
                pipelines.specialize(&mut pipeline_cache, &ortho_hexagon_pipeline, ());

            let draw_hexagon = transparent_2d_draw_functions
                .read()
                .get_id::<DrawHexWorld>()
                .unwrap();

            for (entity, _hex_world) in prepared_hexagons.iter() {
                // commands.entity(entity).insert();

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
}
