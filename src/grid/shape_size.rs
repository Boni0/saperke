use druid::im::Vector;
use druid::{Data, Lens};

use super::GridCellPoint;

pub type GridShapeSizeUnit = usize;

pub type NonExistedPoints = Vector<GridCellPoint>;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Data)]
pub enum GridShape {
    RectangleOrSquare,
    Unusual(NonExistedPoints),
}

#[derive(Clone, PartialEq, Lens, Data)]
pub struct GridSize {
    pub width: GridShapeSizeUnit,
    pub height: GridShapeSizeUnit,
}
