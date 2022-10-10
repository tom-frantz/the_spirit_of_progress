use self::cell::APPROX_ONE_KM_SQUARE_RESOLUTION;
pub use self::{
    cell::{CellData, HexWorldCell},
    iterator::WorldIter,
};
use bevy::prelude::Component;
use h3ron::{res0_cells, H3Cell, Index as H3Index};
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    process::id,
};

pub mod cell;
pub mod iterator;

#[derive(Debug, Component)]
pub struct HexWorldData<T>
where
    T: Debug,
{
    data: Vec<HexWorldCell<T>>,
}

impl<T> HexWorldData<T>
where
    T: Debug,
{
    pub fn new_from_cells<F>(mut new_func: F) -> Self
    where
        F: FnMut(H3Cell) -> T,
    {
        let res0_cells_vec = res0_cells();

        HexWorldData {
            data: res0_cells_vec
                .into_iter()
                .map(|c| HexWorldCell::new(c, new_func(c)))
                .collect(),
        }
    }

    pub fn get(&self, cell_id: H3Cell) -> &HexWorldCell<T> {
        assert!(cell_id.is_valid());

        match cell_id.resolution() {
            0 => {
                for cell in self.data.iter() {
                    if cell.id() == cell_id {
                        return cell;
                    }
                }
                panic!("A valid cell with resolution 0 could not be found in the data.")
            }
            _ => {
                for cell in self.data.iter() {
                    if cell.id().contains(&cell_id).expect("Cell was invalid") {
                        return cell.get(cell_id);
                    }
                }
                panic!("A valid cell could not be found in the child cells.")
            }
        }
    }

    pub fn get_mut(&mut self, cell_id: H3Cell) -> &mut HexWorldCell<T> {
        assert!(cell_id.is_valid());

        match cell_id.resolution() {
            0 => {
                for cell in self.data.iter_mut() {
                    if cell.id() == cell_id {
                        return cell;
                    }
                }
                panic!("A valid cell with resolution 0 could not be found in the data.")
            }
            _ => {
                for cell in self.data.iter_mut() {
                    if cell.id().contains(&cell_id).expect("Cell was invalid") {
                        return cell.get_mut(cell_id);
                    }
                }
                panic!("A valid cell could not be found in the child cells.")
            }
        }
    }

    pub fn iter_at_level(&self, level: u8) -> WorldIter<T> {
        assert!(level <= APPROX_ONE_KM_SQUARE_RESOLUTION);

        WorldIter::at_level(&self, level)
    }
}

impl<T> Index<H3Cell> for HexWorldData<T>
where
    T: Debug,
{
    type Output = HexWorldCell<T>;

    fn index(&self, index: H3Cell) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<H3Cell> for HexWorldData<T>
where
    T: Debug,
{
    fn index_mut(&mut self, index: H3Cell) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Default for HexWorldData<T>
where
    T: Default + Debug,
{
    fn default() -> Self {
        let res0_cells_vec = res0_cells();

        HexWorldData {
            data: res0_cells_vec
                .into_iter()
                .map(|c| HexWorldCell::new(c, T::default()))
                .collect(),
        }
    }
}

impl<T> Clone for HexWorldData<T>
where
    T: Clone + Debug,
{
    fn clone(&self) -> Self {
        HexWorldData {
            data: self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world() {
        let world: HexWorldData<f64> = HexWorldData::default();

        println!("{world:?}");
        assert_eq!(world.data.len(), 122)
    }
}
