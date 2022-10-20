use crate::render::traits::{CellRender, QueryCellRender};
use bevy::prelude::*;
use h3ron::H3Cell;

use super::HexWorldData;

#[derive(Debug, Default, Clone)]
pub struct ElevationData {}

impl CellRender for ElevationData {
    fn cell_colour(&self) -> Color {
        Color::BISQUE
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct WorldElevationData(HexWorldData<ElevationData>);

impl QueryCellRender for WorldElevationData {
    fn cell_colour(&self, cell_id: H3Cell) -> Color {
        self.0.cell_colour(cell_id)
    }
}
