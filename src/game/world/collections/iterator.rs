use super::{Cell, CellData, HexWorld};
use h3ron::H3Cell;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CellIterValue<'a, T>
where
    T: Debug,
{
    id: H3Cell,
    cell: &'a Cell<T>,
}

impl<'a, T> CellIterValue<'a, T>
where
    T: Debug,
{
    pub fn new(id: H3Cell, cell: &'a Cell<T>) -> CellIterValue<'a, T> {
        CellIterValue { id, cell }
    }
}

#[derive(Debug)]
pub struct WorldIter<'a, T>
where
    T: Debug,
{
    world: &'a HexWorld<T>,
    level: u8,

    cursor: H3Cell,
}

impl<'a, T> WorldIter<'a, T>
where
    T: Debug,
{
    pub fn at_level(world: &HexWorld<T>, level: u8) -> WorldIter<T> {
        WorldIter {
            world,
            level,
            cursor: H3Cell::try_from(0).unwrap(),
        }
    }
}

impl<'a, T> Iterator for WorldIter<'a, T>
where
    T: Debug,
{
    type Item = CellIterValue<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let world: HexWorld<f64> = HexWorld::default();

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
