use druid::{Data, Lens};
use druid::im::Vector;

use super::GridCellPoint;

pub type GridShapeSizeUnit = usize;

pub type NonExistedPoints = Vector<GridCellPoint>;

#[derive(Clone, PartialEq, Data)]
pub enum GridShape {
    RectangleOrSquare,
    Unusual(NonExistedPoints)
}

#[derive(Clone, PartialEq, Lens, Data)]
pub struct GridSize {
    pub width: GridShapeSizeUnit,
    pub height: GridShapeSizeUnit
}