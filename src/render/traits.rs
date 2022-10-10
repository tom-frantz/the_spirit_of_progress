use crate::game::world::{CellData, HexWorldCell, HexWorldData};
use bevy::prelude::Color;
use h3ron::{H3Cell, Index};
use std::fmt::Debug;

pub trait QueryCellRender {
    fn cell_colour(&self, cell_id: H3Cell) -> Color;
}

pub trait CellRender {
    fn cell_colour(&self) -> Color;
}

impl<T> CellRender for HexWorldCell<T>
where
    T: CellRender + Debug,
{
    fn cell_colour(&self) -> Color {
        return match self.data() {
            CellData::Data(data) => data.cell_colour(),
            CellData::ChildrenData(children_data) => {
                let self_id = self.id();
                let center_cell_id = self_id
                    .center_child(self_id.resolution() + 1)
                    .expect("This cell should have a center child cell");

                for cell in children_data {
                    if center_cell_id == cell.id() {
                        return cell.cell_colour();
                    }
                }
                panic!("Expected center cell to be a child!")
            }
        };
    }
}

impl<T> QueryCellRender for HexWorldData<T>
where
    T: CellRender + Debug,
{
    fn cell_colour(&self, cell_id: H3Cell) -> Color {
        let cell = self.get(cell_id);
        cell.cell_colour()
    }
}
