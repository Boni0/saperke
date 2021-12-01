use druid::im::Vector;
use druid::{Data, Lens};

use super::{GridCellPoint, GridSizeUnit};

pub type BombsPoints = Vector<GridCellPoint>;

#[derive(Clone, Data, Copy, PartialEq)]
pub enum GridBombsPropagation {
    Randomized,
    Selected,
}

#[derive(Clone, Data, PartialEq)]
pub enum GridBombsConfig {
    Randomized(GridSizeUnit),
    Selected(BombsPoints)
}

#[derive(Clone, Data, Lens, PartialEq)]
pub struct GridBombs {
    pub propagation: GridBombsPropagation,
    pub count: GridSizeUnit,
    pub points: BombsPoints
}