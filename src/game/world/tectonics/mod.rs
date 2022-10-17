use crate::render::traits::{CellRender, QueryCellRender};
use crate::ui::theme::{Colour, Terrain};
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
                Terrain::Sea6,
                Terrain::Sea5,
                Terrain::Sea4,
                Terrain::Sea3,
                Terrain::Sea2,
                Terrain::Sea1,
                Terrain::SeaLevelWater,
                Terrain::SeaLevelLand,
                Terrain::Land1,
                Terrain::Land2,
                Terrain::Land3,
                Terrain::Land4,
                Terrain::Land5,
                Terrain::Land6,
                Terrain::Land7,
                Terrain::Land8,
                Terrain::Land9,
            ]
        };

        let mut colours = random_colours();
        let data = HexWorldData::new_from_cells(|cell_id| {
            let colour = colours.pop();

            if let Some(industry_colour) = colour {
                TectonicsData {
                    colour: industry_colour.color(),
                }
            } else {
                colours = random_colours();
                TectonicsData {
                    colour: colours.pop().unwrap().color(),
                }
            }
        });

        WorldTectonicsData(data)
    }
}
