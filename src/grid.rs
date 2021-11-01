use rand::prelude::*;

type GridShapeVec = Vec<Vec<GridCell>>;
type GridCellValueUnit = u8;

#[derive(PartialEq)]
pub enum GridCellState {
    Hidden,
    Tagged,
    Questioned,
    Visible
}

#[derive(Clone, Copy, PartialEq)]
pub enum GridCellVariant {
    WithValue(GridCellValueUnit),
    WithBomb,
    NonExist
}

pub struct GridCell {
    pub x: usize,
    pub y: usize,
    pub state: GridCellState,
    pub variant: GridCellVariant,
}

pub enum GridShape {
    RectangleOrSquare,
    Unusual
}

pub struct GridStruct {
    pub width: usize,
    pub height: usize,
    pub shape_type: GridShape, // rename to GridShapeType
    pub cells: GridShapeVec, 
    pub cells_count: usize,
    pub tagged_count: usize,
    pub visible_count: usize
}

impl GridStruct {
    pub fn new_rectangle_or_square_grid(height: usize, width: usize) -> GridStruct {
        let mut cells: GridShapeVec = Vec::new();

        for x in 0..width {
            let mut shape_vec_height: Vec<GridCell> = Vec::new();

            for y in 0..height {
                shape_vec_height.push(
                    GridCell { 
                        x,
                        y,
                        state: GridCellState::Hidden, 
                        variant: GridCellVariant::WithValue(0) 
                    }
                );
            }
            
            cells.push(shape_vec_height);
        }

        GridStruct { 
            width,
            height,
            shape_type: GridShape::RectangleOrSquare,
            cells,
            cells_count: width * height,
            tagged_count: 0,
            visible_count: 0
        }
    }

    pub fn get_cell(&mut self, y_cord: usize, x_cord: usize) -> Option<&mut GridCell> {
        match self.cells.get_mut(y_cord) {
            Some(row) => {
                row.get_mut(x_cord)
            },
            None => { None },
        }
    }

    pub fn set_mines_to_cells_randomly(&mut self, mines_count: usize) {
        let mut rng = rand::thread_rng();
        let mines_count = if mines_count > self.cells_count {
            self.cells_count
        } else {
            mines_count
        };

        for _mine in 0..mines_count {
            let mut mine_x: usize;
            let mut mine_y: usize;
            let mut cell: &mut GridCell;

            loop {
                let random_cell = rng.gen_range(0..self.cells_count);
                mine_y = random_cell / self.height;
                mine_x = random_cell % self.height;

                cell = self.get_cell(mine_y, mine_x).unwrap();

                match cell.variant {
                    GridCellVariant::WithValue(_) => break,
                    _ => {}
                };
            }

            cell.variant = GridCellVariant::WithBomb;

            let (first_y, adjustment_y) = if mine_y > 0 { 
                (mine_y - 1, 0)
            } else { 
                (mine_y, 1) 
            };

            let (first_x, adjustment_x) = if mine_x > 0 { 
                (mine_x - 1, 0)
            } else { 
                (mine_x, 1) 
            };

            for add_y in 0..(3 - adjustment_y) {

                for add_x in 0..(3 - adjustment_x) {
                    let search_y = first_y + add_y;
                    let search_x = first_x + add_x;

                    if search_y == mine_y && search_x == mine_x { continue; }

                    let neighbor_cell = match self.get_cell(search_y, search_x) {
                        Some(cell) => { cell },
                        None => { continue; },
                    };

                    match neighbor_cell.variant {
                        GridCellVariant::WithValue(value) => {
                            neighbor_cell.variant = GridCellVariant::WithValue(value + 1);
                        },
                        _ => {},
                    };
                }

            }
        }
    }

    pub fn set_cell_visible(&mut self, y_cord: usize, x_cord: usize) -> Option<GridCellVariant> {
        let mut variant_option = None;
        
        if let Some(cell) = self.get_cell(y_cord, x_cord) {
            if GridCellVariant::NonExist != cell.variant && (
                GridCellState::Hidden == cell.state ||
                GridCellState::Questioned == cell.state
            ) {
                cell.state = GridCellState::Visible;
                variant_option = Some(cell.variant);
            }
        }

        if let Some(variant) = variant_option {
            self.visible_count += 1;

            if GridCellVariant::WithValue(0) == variant {
                if y_cord != 0 {
                    self.set_cell_visible(y_cord - 1, x_cord);
                }
                if y_cord != self.height {
                    self.set_cell_visible(y_cord + 1, x_cord);
                }
    
                if x_cord != 0 {
                    self.set_cell_visible(y_cord, x_cord - 1);
                }
                if x_cord != self.width {
                    self.set_cell_visible(y_cord, x_cord + 1);
                }
            }
        }

        variant_option
    }

    pub fn toggle_cell_tagged_state(&mut self, y_cord: usize, x_cord: usize) {
        if let Some(cell) = self.get_cell(y_cord, x_cord) {
            if GridCellVariant::NonExist != cell.variant {
                match cell.state {
                    GridCellState::Hidden => {
                        cell.state = GridCellState::Tagged;
                        self.tagged_count += 1;
                    },
                    GridCellState::Tagged => {
                        cell.state = GridCellState::Questioned;
                        self.tagged_count -= 1;
                    },
                    GridCellState::Questioned => {
                        cell.state = GridCellState::Hidden;
                        self.tagged_count -= 1;
                    },
                    _ => {},
                }
            }
        }
    }
    
}