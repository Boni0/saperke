mod cells;
mod bombs;
mod shape_size;

use rand::prelude::*;

use druid::im::Vector;
use druid::{Data, Lens};

pub type GridSizeUnit = usize;

pub use cells::{
    GridCells,
    GridCell,
    GridCellPoint,
    GridCellMatrix,
    GridCellMatrixRow,
    GridCellState,
    GridCellOpenedState,
    GridCellFlaggedState,
    GridCellVariant,
    GridCellValueUnit,
    GridExistingCell,
    GridCellValue
};

pub use shape_size::{
    GridShape,
    GridSize,
    GridShapeSizeUnit,
    NonExistedPoints,
};

pub use bombs::{
    GridBombs,
    GridBombsConfig,
    GridBombsPropagation,
    BombsPoints
};

#[derive(Clone, Data, Lens)]
pub struct Grid {
    pub size: GridSize,
    pub shape: GridShape,
    pub cells: GridCells,
    pub bombs: GridBombs
}

const INIT_EXISTING_CELL: GridExistingCell = GridExistingCell {
    value: GridCellValue::Number(0),
    state: GridCellState::Idle,
    is_visible: false,
};

impl Grid {
    pub fn new(size: GridSize, shape: GridShape, bombs_config: &GridBombsConfig) -> Self {
        let mut exist_count = size.height * size.width;
        let mut non_existing_points_option = None;

        match &shape {
            GridShape::Unusual(non_existing_points) => {
                exist_count -= non_existing_points.len();
                non_existing_points_option = Some(non_existing_points);
            },
            _ => ()
        };

        let cells = GridCells {
            matrix: Grid::create_cells_matrix(&size, non_existing_points_option),
            all_count: size.height * size.width,
            exist_count,
            visible_count: 0,
            tagged_points: Vector::new(),
            questioned_points: Vector::new(),
        };

        let get_eligible_bombs_count = |check_amount: usize| {
            if check_amount >= cells.exist_count {
                cells.exist_count - 1
            } else {
                check_amount
            }
        };

        let bombs = match bombs_config {
            GridBombsConfig::Randomized(amount) => GridBombs {
                count: get_eligible_bombs_count(*amount),
                points: Vector::new(),
                propagation: GridBombsPropagation::Randomized
            },
            GridBombsConfig::Selected(bomb_points) => {
                let count = get_eligible_bombs_count(bomb_points.len());
                let difference_in_count = bomb_points.len() - count;

                let mut bomb_points = bomb_points.clone();

                for _ in 0..difference_in_count {
                    bomb_points.pop_back();
                }

                GridBombs {
                    count,
                    points: bomb_points,
                    propagation: GridBombsPropagation::Selected
                }
            },
        };

        let mut grid = Self {
            size,
            shape,
            cells,
            bombs,
        };

        let random_mines_count = match grid.bombs.propagation {
            GridBombsPropagation::Randomized => grid.bombs.count,
            GridBombsPropagation::Selected => {
                let mut new_selected_points: BombsPoints = Vector::new();

                for point in grid.bombs.points.clone() {
                    if let Ok(()) = grid.set_bomb_to_cell(&point) {
                        new_selected_points.push_back(point.clone());
                    }
                }

                grid.bombs.points = new_selected_points;
                grid.bombs.count
            },
        };

        grid.set_bombs_to_grid_randomly(random_mines_count);

        grid
    }

    pub fn refresh(&mut self) {
        self.cells.visible_count = 0;
        self.cells.tagged_points = Vector::new();
        self.cells.questioned_points = Vector::new();

        for row in self.cells.matrix.iter_mut() {
            for cell in (row as &mut GridCellMatrixRow).iter_mut() {
                let cell = cell as &mut GridCell;

                if let GridCellVariant::Exist(cell_data) = &cell.variant {
                    cell.variant = GridCellVariant::Exist(
                        match self.bombs.propagation {
                            GridBombsPropagation::Randomized => INIT_EXISTING_CELL,
                            GridBombsPropagation::Selected => GridExistingCell {
                                value: cell_data.value,
                                ..INIT_EXISTING_CELL
                            }
                        }
                    );
                } 
            }
        }

        if let GridBombsPropagation::Randomized = self.bombs.propagation {
            self.set_bombs_to_grid_randomly(self.bombs.count)
        }
    }

    pub fn get_cell_mut(&mut self, point: &GridCellPoint) -> Option<&mut GridCell> {
        match self.cells.matrix.get_mut(point.y) {
            Some(row) => {
                row.get_mut(point.x)
            },
            None => { None },
        }
    }

    pub fn get_existing_cell_mut(&mut self, point: &GridCellPoint) -> Option<&mut GridExistingCell> {
        self
            .get_cell_mut(point)
            .and_then(|cell| {
                match &mut cell.variant {
                    GridCellVariant::NonExist => None,
                    GridCellVariant::Exist(cell_data) => Some(cell_data),
                }
            })
    }

    pub fn set_idle_cells_visible(&mut self, start_point: &GridCellPoint) -> Option<GridCellValue> {
        let GridCellPoint { y: y_cord, x: x_cord } = *start_point;
        let mut start_point_cell_value_option = None;

        if let Some(cell_data) = self.get_existing_cell_mut(start_point) {
            if 
                cell_data.is_visible == false
                && cell_data.state == GridCellState::Idle
            {
                cell_data.is_visible = true;
                start_point_cell_value_option = Some(cell_data.value);
            }
        }

        if let Some(value) = start_point_cell_value_option {
            self.cells.visible_count += 1;

            if GridCellValue::Number(0) == value {
                if y_cord > 0 {
                    self.set_idle_cells_visible(&GridCellPoint {
                        y: y_cord - 1, 
                        x: x_cord
                    });
                }

                if y_cord < self.size.height {
                    self.set_idle_cells_visible(&GridCellPoint {
                        y: y_cord + 1, 
                        x: x_cord
                    });
                }
    
                if x_cord > 0 {
                    self.set_idle_cells_visible(&GridCellPoint {
                        y: y_cord, 
                        x: x_cord - 1
                    });
                }

                if x_cord < self.size.width {
                    self.set_idle_cells_visible(&GridCellPoint {
                        y: y_cord, 
                        x: x_cord + 1
                    });
                }
            }
        }

        start_point_cell_value_option
    }
    
    // pub fn set_cell_state(&mut self, point: &GridCellPoint, state: GridCellState) {
    //     if let Some(cell_data) = self.get_existing_cell_mut(point) {
    //         if !cell_data.is_visible {
    //             cell_data.state = state;
    //         }
    //     }
    // }

    pub fn set_all_bombs_visible(&mut self) {
        for bomb_point in self.bombs.points.clone() {
            if let Some(cell_data) = self.get_existing_cell_mut(&bomb_point) {
                cell_data.is_visible = true;
            }
        }
    }

    pub fn set_all_flagged_cells_to_verify(&mut self) {
        for flagged_point in self.cells.tagged_points.clone().iter().chain(self.cells.questioned_points.clone().iter()) {
            if let Some(cell_data) = self.get_existing_cell_mut(&flagged_point) {
                if let GridCellState::Flagged(flagged_state) = &cell_data.state {
                    cell_data.state = GridCellState::ToVerifyFlag(flagged_state.clone());
                }
            }
        }
    }

    pub fn set_cell_flagged_state(&mut self, point: &GridCellPoint, option_flagged_state: Option<GridCellFlaggedState>) {
        let mut next_cell_state = None;

        if let Some(cell_data) = self.get_existing_cell_mut(point) {
            if !cell_data.is_visible {
                match option_flagged_state {
                    Some(flagged_state) => Some(GridCellState::Flagged(flagged_state)),
                    None => Some(GridCellState::Idle),
                }
                .and_then(|new_state| {
                    cell_data.state = new_state.clone();
                    next_cell_state = Some(new_state);
                    Some(())
                });
            }
        }

        match next_cell_state {
            Some(GridCellState::Flagged(GridCellFlaggedState::Tagged)) => {
                self.cells.tagged_points.push_back(point.clone());
            },
            Some(GridCellState::Flagged(GridCellFlaggedState::Questioned)) => {
                if let Ok(index) = self.cells.tagged_points.binary_search(point) {
                    self
                        .cells
                        .tagged_points
                        .remove(index);
                }

                self
                    .cells
                    .questioned_points
                    .push_back(point.clone());
            },
            Some(GridCellState::Idle) => {
                if let Ok(index) = self.cells.questioned_points.binary_search(point) {
                    self
                        .cells
                        .questioned_points
                        .remove(index);
                }
            },
            _ => (),
        }
    }

    fn create_cells_matrix(GridSize { height, width }: &GridSize, non_existing_points: Option<&NonExistedPoints>) -> GridCellMatrix {
        let mut cells: GridCellMatrix = Vector::new();

        let is_point_exist = |point: &GridCellPoint| {
            if let Some(points_vec) = non_existing_points {
                for non_exist_point in points_vec {
                    let non_exist_point = non_exist_point as &GridCellPoint;

                    if
                        non_exist_point.y == point.y &&
                        non_exist_point.x == point.x
                    {
                        return false
                    }
                }
            }

            true
        };

        for y in 0..*height {
            let mut shape_vec_height: GridCellMatrixRow = Vector::new();

            for x in 0..*width {
                let current_point = GridCellPoint { y, x };

                shape_vec_height.push_back(
                    GridCell { 
                        variant: if is_point_exist(&current_point) {
                            GridCellVariant::Exist(INIT_EXISTING_CELL)
                        } else {
                            GridCellVariant::NonExist
                        },
                        point: current_point,
                    }
                );
            }
            
            cells.push_back(shape_vec_height);
        }

        cells
    }

    fn set_bomb_to_cell(&mut self, point: &GridCellPoint) -> Result<(), ()> {
        if let Some(cell_data) = self.get_existing_cell_mut(point) {
            if let GridCellValue::Number(_) = cell_data.value {
                cell_data.value = GridCellValue::Bomb;

                let (first_y, adjustment_y) = if point.y > 0 { (point.y - 1, 0) } else { (point.y, 1) };
                let (first_x, adjustment_x) = if point.x > 0 { (point.x - 1, 0) } else { (point.x, 1) };

                for add_y in 0..(3 - adjustment_y) {
                    for add_x in 0..(3 - adjustment_x) {
                        let search_y = first_y + add_y;
                        let search_x = first_x + add_x;
                    
                        if search_y == point.y && search_x == point.x { continue; }

                        let neighbor_cell = match self.get_cell_mut(&GridCellPoint { x: search_x, y: search_y }) {
                            Some(cell) => { cell },
                            None => { continue; },
                        };

                        match &mut neighbor_cell.variant {
                            GridCellVariant::Exist(neighbor_cell_data) => {
                                if let GridCellValue::Number(amount) = neighbor_cell_data.value {
                                    neighbor_cell_data.value = GridCellValue::Number(amount + 1);
                                }
                            },
                            _ => {},
                        };
                    }
                }

                return Ok(())
            }
        }

        Err(())
    }

    fn set_bombs_to_grid_randomly(&mut self, mines_count: usize) {
        let mut rng = rand::thread_rng();
        let mut mines_created = 0;

        while mines_created < mines_count {
            let random_cell = rng.gen_range(0..self.cells.all_count);
            let mine_point = GridCellPoint {
                y: random_cell / self.size.height,
                x: random_cell % self.size.height,
            };
           
            if Ok(()) == self.set_bomb_to_cell(&mine_point) {
                self.bombs.points.push_back(mine_point);
                mines_created += 1;
            }
        }
    }

}