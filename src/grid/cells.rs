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

pub trait RemovePoint {
    fn remove_point(&mut self, point: &GridCellPoint);
}

impl RemovePoint for Vector<GridCellPoint> {
    fn remove_point(&mut self, point: &GridCellPoint) {
        if let Ok(index) = self.binary_search(point) {
            self.remove(index);
        }
    }
}

impl GridCells {
    pub fn reset_flagged_and_visible(&mut self) {
        self.visible_count = 0;
        self.tagged_points = Vector::new();
        self.questioned_points = Vector::new();
    }

    pub fn get_existing_cell(&mut self, point: &GridCellPoint) -> Option<&mut GridExistingCell> {
        match self.matrix.get_mut(point.y) {
            Some(row) => {
                row.get_mut(point.x)
            },
            None => { None },
        }
        .and_then(|cell| {
            match &mut cell.variant {
                GridCellVariant::NonExist => None,
                GridCellVariant::Exist(cell_data) => Some(cell_data),
            }
        })
    }

    pub fn set_cell_state(&mut self, point: &GridCellPoint, state: GridCellState) {
        if let Some(cell_data) = self.get_existing_cell(point) {
            cell_data.state = state;
        }
    }

    pub fn set_cell_state_to_verify(&mut self, point: &GridCellPoint) {
        if let Some(cell_data) = self.get_existing_cell(point) {
            match &cell_data.state {
                GridCellState::Flagged(flagged_state) => {
                    cell_data.state = GridCellState::ToVerifyFlag(flagged_state.clone());
                },
                _ => ()
            }
        }
    }
} 