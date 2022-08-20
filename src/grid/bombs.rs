use druid::im::Vector;
use druid::{Data, Lens};

use super::{GridCellPoint, SizeUnit};

pub type BombsPoints = Vector<GridCellPoint>;

#[derive(Clone, Data, Copy, PartialEq)]
pub enum GridBombsPropagation {
    Randomized,
    Selected,
}

#[allow(dead_code)]
#[derive(Clone, Data, PartialEq)]
pub enum GridBombsConfig {
    Randomized(SizeUnit),
    Selected(BombsPoints),
}

#[derive(Clone, Data, Lens, PartialEq)]
pub struct GridBombs {
    pub propagation: GridBombsPropagation,
    pub count: SizeUnit,
    pub points: BombsPoints,
}
