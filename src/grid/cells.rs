use druid::{Data, Lens};
use druid::im::Vector;

use super::GridSizeUnit;

pub type GridCellValueUnit = u8;

pub type GridCellMatrixRow = Vector<GridCell>;
pub type GridCellMatrix = Vector<GridCellMatrixRow>;

#[derive(Clone, PartialEq, Data, Lens, PartialOrd, Eq, Ord)]
pub struct GridCellPoint {
    pub x: GridSizeUnit,
    pub y: GridSizeUnit,
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellFlaggedState {
    Tagged,
    Questioned
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellOpenedState {
    NoAction,
    CausedLoss,
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellState {
    Idle,
    Active,
    Flagged(GridCellFlaggedState),
    Opened(GridCellOpenedState),
    ToVerifyFlag(GridCellFlaggedState),
}

#[derive(Clone, Data, Copy, PartialEq)]
pub enum GridCellValue {
    Number(GridCellValueUnit),
    Bomb,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridExistingCell {
    pub value: GridCellValue,
    pub state: GridCellState,
    pub is_visible: bool,
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellVariant {
    NonExist,
    Exist(GridExistingCell)
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCell {
    pub point: GridCellPoint,
    pub variant: GridCellVariant,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCellsTagged {
    points: Vector<GridCellPoint>,
    count: GridSizeUnit
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCells {
    pub matrix: GridCellMatrix,
    pub all_count: GridSizeUnit,
    pub exist_count: GridSizeUnit,
    pub visible_count: GridSizeUnit,
    pub tagged_points: Vector<GridCellPoint>,
    pub questioned_points: Vector<GridCellPoint>,
}