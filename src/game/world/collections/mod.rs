use self::cell::APPROX_ONE_KM_SQUARE_LEVEL;
pub use self::cell::{Cell, CellData};
pub use self::iterator::WorldIter;
use s2::cellid::CellID;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};

pub mod cell;
pub mod iterator;

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
    pub fn new(data: [CellData<T>; 6]) -> Self {
        World {
            data: data
                .into_iter()
                .enumerate()
                .map(|(index, cell)| {
                    Cell::new_with_cell_data(CellID::from_face(index as u64), cell)
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
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

    pub fn iter_at_level(&self, level: u64) -> WorldIter<T> {
        assert!(level <= APPROX_ONE_KM_SQUARE_LEVEL);

        WorldIter::at_level(&self, level)
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

impl<T> Clone for World<T>
where
    T: Clone + Debug,
{
    fn clone(&self) -> Self {
        World {
            data: self.data.clone(),
        }
    }
}
