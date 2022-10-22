use h3ron::{H3Cell, Index as CellIndex};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub const _APPROX_FIVE_KM_SQUARE_RESOLUTION: u8 = 7; // 5.161293360
pub const APPROX_ONE_KM_SQUARE_RESOLUTION: u8 = 8; // 0.737327598

#[derive(Debug)]
pub enum CellData<T>
where
    T: Debug,
{
    Data(T),
    ChildrenData(Vec<HexWorldCell<T>>),
}

impl<T> Clone for CellData<T>
where
    T: Clone + Debug,
{
    fn clone(&self) -> Self {
        match self {
            CellData::Data(t) => CellData::Data(t.clone()),
            CellData::ChildrenData(t) => CellData::ChildrenData(t.clone()),
        }
    }
}

#[derive(Debug)]
pub struct HexWorldCell<T>
where
    T: Debug,
{
    id: H3Cell,
    data: CellData<T>,
}

impl<T> HexWorldCell<T>
where
    T: Debug,
{
    pub fn new(id: H3Cell, data: T) -> Self {
        assert!(id.resolution() <= APPROX_ONE_KM_SQUARE_RESOLUTION);

        HexWorldCell {
            id,
            data: CellData::Data(data),
        }
    }

    pub fn new_with_cell_data(id: H3Cell, data: CellData<T>) -> Self {
        assert!(id.resolution() <= APPROX_ONE_KM_SQUARE_RESOLUTION);

        HexWorldCell { id, data }
    }

    pub fn id(&self) -> H3Cell {
        self.id
    }
    pub fn data(&self) -> &CellData<T> {
        return &self.data;
    }

    // Can assume that `cell_id` is valid
    pub fn get(&self, cell_id: H3Cell) -> &HexWorldCell<T> {
        let self_resolution = self.id.resolution();

        return match cell_id.resolution() {
            res if res == self_resolution => self,
            _ => match &self.data {
                // return self if this is as far as the data goes
                CellData::Data(_) => self,
                // If the data continues, go deeper
                CellData::ChildrenData(children_cells) => {
                    for cell in children_cells {
                        if cell.id.contains(&cell_id).expect("Cell was invalid") {
                            return cell.get(cell_id);
                        }
                    }
                    panic!("Valid cell used but could not find a sub cell that contains it.")
                }
            },
        };
    }

    pub fn get_mut(&mut self, cell_id: H3Cell) -> &mut HexWorldCell<T> {
        let self_resolution = self.id.resolution();

        match cell_id.resolution() {
            // Handle at the correct resolution
            x if self_resolution == x => self,
            _ => {
                // Handle if the data ends here. (incorrect resolution)
                if let CellData::Data(_) = self.data {
                    return self;
                }

                match &mut self.data {
                    // Handle if the data keeps going down. (incorrect resolution)
                    CellData::ChildrenData(children_cells) => {
                        for cell in children_cells {
                            if cell.id.contains(&cell_id).expect("Cell was invalid") {
                                return cell.get_mut(cell_id);
                            }
                        }
                        panic!("Valid cell used but could not find a sub cell that contains it.")
                    }
                    // Don't use `_` here incase I add extra variations to `CellData`
                    CellData::Data(_) => panic!("This should never happen; It's covered above."),
                }
            }
        }
    }
}

impl<T> Deref for HexWorldCell<T>
where
    T: Debug,
{
    type Target = CellData<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for HexWorldCell<T>
where
    T: Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Index<H3Cell> for HexWorldCell<T>
where
    T: Debug,
{
    type Output = HexWorldCell<T>;

    fn index(&self, index: H3Cell) -> &Self::Output {
        assert!(index.is_valid());

        self.get(index)
    }
}

impl<T> IndexMut<H3Cell> for HexWorldCell<T>
where
    T: Debug,
{
    fn index_mut(&mut self, index: H3Cell) -> &mut Self::Output {
        assert!(index.is_valid());

        self.get_mut(index)
    }
}

impl<T> Clone for HexWorldCell<T>
where
    T: Clone + Debug,
{
    fn clone(&self) -> Self {
        HexWorldCell {
            id: self.id,
            data: self.data.clone(),
        }
    }
}
