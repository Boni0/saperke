use druid::{Data, Lens};
use druid::im::Vector;

pub type GridCellValueUnit = u8;

pub type GridCellMatrixRow = Vector<GridCell>;
pub type GridCellMatrix = Vector<GridCellMatrixRow>;

#[derive(PartialEq, Data, Clone)]
pub enum GridCellState {
    Hidden,
    Tagged,
    Questioned,
    Active,
    Visible
}

#[derive(Clone, Data, Copy, PartialEq)]
pub enum GridCellVariant {
    WithValue(GridCellValueUnit),
    WithBomb,
    NonExist
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
    pub state: GridCellState,
    pub variant: GridCellVariant,
}