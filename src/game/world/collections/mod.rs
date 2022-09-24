pub use self::cell::{Cell, CellData};
use s2::cellid::CellID;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

pub mod cell;

#[derive(Debug)]
pub struct World<T>
where
    T: Debug,
{
    data: [Cell<T>; 6],
}

impl<T> World<T>
where
    T: Debug,
{
    pub fn new(data: [Cell<T>; 6]) -> Self {
        World { data }
    }

    pub fn get(&self, cell_id: CellID) -> &Cell<T> {
        assert!(cell_id.is_valid());

        let face = cell_id.face();
        &self.data[face as usize].get(cell_id)
    }

    pub fn get_mut(&mut self, cell_id: CellID) -> &mut Cell<T> {
        assert!(cell_id.is_valid());

        let face = cell_id.face();
        self.data[face as usize].get_mut(cell_id)
    }
}

impl<T> Index<CellID> for World<T>
where
    T: Debug,
{
    type Output = Cell<T>;

    fn index(&self, index: CellID) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<CellID> for World<T>
where
    T: Debug,
{
    fn index_mut(&mut self, index: CellID) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<T> Default for World<T>
where
    T: Default + Debug,
{
    fn default() -> Self {
        World {
            data: [
                Cell::new(CellID::from_face(0), T::default()),
                Cell::new(CellID::from_face(1), T::default()),
                Cell::new(CellID::from_face(2), T::default()),
                Cell::new(CellID::from_face(3), T::default()),
                Cell::new(CellID::from_face(4), T::default()),
                Cell::new(CellID::from_face(5), T::default()),
            ],
        }
    }
}
