use druid::im::Vector;
use druid::{Data, Lens};

use super::SizeUnit;

pub type GridCellValueUnit = u8;

pub type GridCellMatrixRow = Vector<GridCell>;
pub type GridCellMatrix = Vector<GridCellMatrixRow>;
pub type NonExistedPoints = Vector<GridCellPoint>;

#[derive(Clone, PartialEq, Data, Lens, PartialOrd, Eq, Ord)]
pub struct GridCellPoint {
    pub x: SizeUnit,
    pub y: SizeUnit,
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellFlaggedState {
    Tagged,
    Questioned,
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

impl GridExistingCell {
    pub const INIT_EXISTING_CELL: GridExistingCell = GridExistingCell {
        value: GridCellValue::Number(0),
        state: GridCellState::Idle,
        is_visible: false,
    };
}

#[derive(PartialEq, Data, Clone)]
pub enum GridCellVariant {
    NonExist,
    Exist(GridExistingCell),
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCell {
    pub point: GridCellPoint,
    pub variant: GridCellVariant,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCellsTagged {
    points: Vector<GridCellPoint>,
    count: SizeUnit,
}

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GridCells {
    pub matrix: GridCellMatrix,
    pub all_count: SizeUnit,
    pub exist_count: SizeUnit,
    pub visible_count: SizeUnit,
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
            Some(row) => row.get_mut(point.x),
            None => None,
        }
        .and_then(|cell| match &mut cell.variant {
            GridCellVariant::NonExist => None,
            GridCellVariant::Exist(cell_data) => Some(cell_data),
        })
    }

    #[allow(dead_code)]
    pub fn get_existing_visible_cell(
        &mut self,
        point: &GridCellPoint,
    ) -> Option<&mut GridExistingCell> {
        self.get_existing_cell(point).and_then(
            |cell| {
                if cell.is_visible {
                    Some(cell)
                } else {
                    None
                }
            },
        )
    }

    pub fn get_existing_invisible_cell(
        &mut self,
        point: &GridCellPoint,
    ) -> Option<&mut GridExistingCell> {
        self.get_existing_cell(point).and_then(
            |cell| {
                if !cell.is_visible {
                    Some(cell)
                } else {
                    None
                }
            },
        )
    }

    #[allow(dead_code)]
    pub fn set_cell_state(&mut self, point: &GridCellPoint, state: GridCellState) -> &mut Self {
        if let Some(cell_data) = self.get_existing_invisible_cell(point) {
            cell_data.state = state;
        }

        self
    }

    pub fn set_cell_idle_state(&mut self, point: &GridCellPoint) -> &mut Self {
        self.get_existing_invisible_cell(point).and_then(|cell| {
            if cell.state == GridCellState::Active {
                cell.state = GridCellState::Idle;
            }
            Some(())
        });

        self
    }

    pub fn set_cell_active_state(&mut self, point: &GridCellPoint) -> &mut Self {
        self.get_existing_invisible_cell(point).and_then(|cell| {
            if cell.state == GridCellState::Idle {
                cell.state = GridCellState::Active;
            }
            Some(())
        });

        self
    }

    pub fn toggle_cell_flagged_state(&mut self, point: &GridCellPoint) -> &mut Self {
        let mut next_cell_state = None;

        if let Some(cell) = self.get_existing_invisible_cell(point) {
            next_cell_state = match cell.state {
                GridCellState::Idle => Some(GridCellState::Flagged(GridCellFlaggedState::Tagged)),
                GridCellState::Flagged(GridCellFlaggedState::Tagged) => {
                    Some(GridCellState::Flagged(GridCellFlaggedState::Questioned))
                }
                GridCellState::Flagged(GridCellFlaggedState::Questioned) => {
                    Some(GridCellState::Idle)
                }
                _ => None,
            }
            .and_then(|next_state| {
                cell.state = next_state.clone();
                Some(next_state)
            });
        }

        match next_cell_state {
            Some(GridCellState::Flagged(GridCellFlaggedState::Tagged)) => {
                self.tagged_points.push_back(point.clone());
            }
            Some(GridCellState::Flagged(GridCellFlaggedState::Questioned)) => {
                self.tagged_points.remove_point(point);
                self.questioned_points.push_back(point.clone());
            }
            Some(GridCellState::Idle) => {
                self.questioned_points.remove_point(point);
            }
            _ => (),
        }

        self
    }

    pub fn set_cell_state_to_verify(&mut self, point: &GridCellPoint) -> &mut Self {
        if let Some(cell_data) = self.get_existing_cell(point) {
            match &cell_data.state {
                GridCellState::Flagged(flagged_state) => {
                    cell_data.state = GridCellState::ToVerifyFlag(flagged_state.clone());
                }
                _ => (),
            }
        }

        self
    }
}
