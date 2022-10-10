use crate::render::traits::CellRender;
use bevy::prelude::*;

use super::HexWorldData;

#[derive(Debug, Default)]
pub struct ElevationData {}

impl CellRender for ElevationData {
    fn cell_colour(&self) -> Color {
        Color::BISQUE
    }
}

#[derive(Component, Debug, Default)]
pub struct WorldElevationData(HexWorldData<ElevationData>);
