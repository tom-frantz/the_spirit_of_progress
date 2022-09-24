use s2::cellid::CellID;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum CellData<T>
where
    T: Debug,
{
    Data(T),
    ChildrenData(Box<[Cell<T>; 4]>),
}

#[derive(Debug)]
pub struct Cell<T>
where
    T: Debug,
{
    id: CellID,
    data: CellData<T>,
}

impl<T> Cell<T>
where
    T: Debug,
{
    pub fn new(id: CellID, data: T) -> Self {
        Cell {
            id,
            data: CellData::Data(data),
        }
    }

    // Can assume that `cell_id` is valid
    pub fn get(&self, cell_id: CellID) -> &Cell<T> {
        let self_level = self.id.level();

        if self_level == cell_id.level() {
            return &self;
        } else if self_level < cell_id.level() {
            // If the target cell is further down the hierarchy,
            return if let CellData::ChildrenData(children_data) = &self.data {
                // ... and there are children cells further down the hierarchy, continue recursively
                let children_level = cell_id.child_position(self_level);
                &children_data[children_level as usize].get(cell_id)
            } else {
                // ... but the data at this level is homogenous, return data.
                &self
            };
        } else {
            panic!("The recursion has gone further than the cell!!!")
        }
    }

    pub fn get_mut(&mut self, cell_id: CellID) -> &mut Cell<T> {
        let self_level = self.id.level();

        if self_level == cell_id.level() {
            return self;
        } else if self_level < cell_id.level() {
            // If the target `cell_id` is further down the hierarchy than `self` ...
            if let CellData::Data(_) = &mut self.data {
                // ... but the data is the same from here.
                return self;
            } else if let CellData::ChildrenData(children_data) = &mut self.data {
                // ... and the data forks continuing down
                return children_data[cell_id.child_position(self_level) as usize].get_mut(cell_id);
            } else {
                panic!("There shouldn't be another enum here!!!")
            };
        } else {
            panic!("The recursion has gone further than the cell!!!")
        }
    }
}

impl<T> Deref for Cell<T>
where
    T: Debug,
{
    type Target = CellData<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Cell<T>
where
    T: Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}