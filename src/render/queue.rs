use crate::render::pipeline::OrthographicHexagonPipeline;
use crate::render::{DrawHexWorld, HexWorldId};
use bevy::core_pipeline::core_2d::Transparent2d;
use bevy::prelude::{Entity, Query, Res, ResMut};
use bevy::render::render_phase::{DrawFunctions, RenderPhase};
use bevy::render::render_resource::{PipelineCache, SpecializedRenderPipelines};
use bevy::render::view::ExtractedView;
use bevy::utils::FloatOrd;

pub fn queue(
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
