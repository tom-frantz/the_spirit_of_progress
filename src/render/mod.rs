use bevy::render::RenderStage;
use bevy::{prelude::*, render::RenderApp};

mod pipeline;
mod shader;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .add_system_to_stage(RenderStage::Extract, extract)
            .add_system_to_stage(RenderStage::Prepare, prepare)
            .add_system_to_stage(RenderStage::Queue, queue);
    }
}

fn extract() {}

fn prepare() {}

fn queue() {}
