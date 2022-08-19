#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ViewMode {
    TectonicPlates,
    TectonicPlateTypes,
    Heights,
}

pub enum MapModeEvent {
    SetViewMode(ViewMode),
}
