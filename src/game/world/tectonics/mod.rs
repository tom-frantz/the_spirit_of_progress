use crate::render::traits::{CellRender, QueryCellRender};
use bevy::prelude::*;
use h3ron::H3Cell;

use super::HexWorldData;

#[derive(Debug, Default, Clone)]
pub struct TectonicsData {
    colour: Color,
}

impl CellRender for TectonicsData {
    fn cell_colour(&self) -> Color {
        self.colour
    }
}

#[derive(Component, Debug, Clone)]
pub struct WorldTectonicsData(HexWorldData<TectonicsData>);

impl QueryCellRender for WorldTectonicsData {
    fn cell_colour(&self, cell_id: H3Cell) -> Color {
        self.0.cell_colour(cell_id)
    }
}

impl Default for WorldTectonicsData {
    fn default() -> Self {
        let random_colours = || {
            vec![
                Color::BLUE,
                Color::GREEN,
                Color::RED,
                Color::PINK,
                Color::PURPLE,
                Color::YELLOW,
                Color::ORANGE,
                Color::OLIVE,
            ]
        };

        let mut colours = random_colours();
        let data = HexWorldData::new_from_cells(|cell_id| {
            let colour = colours.pop();

            if let Some(colour) = colour {
                TectonicsData { colour }
            } else {
                colours = random_colours();
                TectonicsData {
                    colour: colours.pop().unwrap(),
                }
            }
        });

        WorldTectonicsData(data)
    }
}
