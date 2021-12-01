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
            tagged_count: 0,
            visible_count: 0,
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
        self.cells.tagged_count = 0;
        self.cells.visible_count = 0;

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

    pub fn set_cells_visible(&mut self, start_point: &GridCellPoint) -> Option<GridCellValue> {
        let GridCellPoint { y: y_cord, x: x_cord } = *start_point;
        let mut start_point_cell_value_option = None;
        
        if let Some(cell) = self.get_cell_mut(start_point) {
            if let GridCellVariant::Exist(cell_data) = &mut cell.variant {
                if !cell_data.is_visible {
                    cell_data.is_visible = true;
                    start_point_cell_value_option = Some(cell_data.value);
                }
            }
        }

        if let Some(value) = start_point_cell_value_option {
            self.cells.visible_count += 1;

            if GridCellValue::Number(0) == value {
                if y_cord > 0 {
                    self.set_cells_visible(&GridCellPoint {
                        y: y_cord - 1, 
                        x: x_cord
                    });
                }

                if y_cord < self.size.height {
                    self.set_cells_visible(&GridCellPoint {
                        y: y_cord + 1, 
                        x: x_cord
                    });
                }
    
                if x_cord > 0 {
                    self.set_cells_visible(&GridCellPoint {
                        y: y_cord, 
                        x: x_cord - 1
                    });
                }

                if x_cord < self.size.width {
                    self.set_cells_visible(&GridCellPoint {
                        y: y_cord, 
                        x: x_cord + 1
                    });
                }
            }
        }

        start_point_cell_value_option
    }
    
    pub fn set_cell_state(&mut self, point: &GridCellPoint, state: GridCellState) {
        if let Some(cell) = self.get_cell_mut(point) {
            if let GridCellVariant::Exist(cell_data) = &mut cell.variant {
                if !cell_data.is_visible {
                    cell_data.state = state;
                }
            }
        }
    }

    fn create_cells_matrix(GridSize { height, width }: &GridSize, non_existing_points: Option<&NonExistedPoints>) -> GridCellMatrix {
        let mut cells: GridCellMatrix = Vector::new();

        let is_point_exist = |point: &GridCellPoint| {
            if let Some(points_vec) = non_existing_points {
                return points_vec.contains(point)
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
        if let Some(cell) = self.get_cell_mut(point) {

            if let GridCellVariant::Exist(cell_data) = &mut cell.variant {

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