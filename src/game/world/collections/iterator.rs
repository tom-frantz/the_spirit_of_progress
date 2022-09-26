use super::{Cell, CellData, World};
use s2::cellid::CellID;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CellIterValue<'a, T>
where
    T: Debug,
{
    id: CellID,
    cell: &'a Cell<T>,
}

impl<'a, T> CellIterValue<'a, T>
where
    T: Debug,
{
    pub fn new(id: CellID, cell: &'a Cell<T>) -> CellIterValue<'a, T> {
        CellIterValue { id, cell }
    }
}

#[derive(Debug)]
pub struct WorldIter<'a, T>
where
    T: Debug,
{
    world: &'a World<T>,
    level: u64,

    cursor: CellID,
}

impl<'a, T> WorldIter<'a, T>
where
    T: Debug,
{
    pub fn at_level(world: &World<T>, level: u64) -> WorldIter<T> {
        WorldIter {
            world,
            level,
            cursor: CellID::from_face_pos_level(0, 0, level),
        }
    }
}

impl<'a, T> Iterator for WorldIter<'a, T>
where
    T: Debug,
{
    type Item = CellIterValue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.cursor.is_valid() {
            return None;
        }

        let current = self.cursor;
        let next = self.cursor.next();

        let cell = &self.world[self.cursor];

        self.cursor = next;
        return Some(CellIterValue::new(current, cell));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let world: World<f64> = World::default();

        let mut iter = world.iter_at_level(2);

        loop {
            let next = iter.next();

            if let Some(cell) = next {
                println!("{cell:?}");
            } else {
                break;
            }
        }
    }
}
