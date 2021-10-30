use rand::prelude::*;

type GridShapeVec = Vec<Vec<GridCell>>;
type GridCellValueUnit = u8;

#[derive(PartialEq)]
pub enum GridCellState {
    Hidden,
    Tagged,
    Visible
}

pub enum GridCellVariant {
    // WithValue(GridCellValue),
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
}

impl GridStruct {
    pub fn new_rectangle_or_square_grid(width: usize, height: usize) -> GridStruct {
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
            cells_count: width * height
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

    pub fn set_cell_visible<'a>(&'a mut self, cell: &mut GridCell /* y_cord: usize, x_cord: usize*/) /*-> Option<&GridCell>*/  {
        // let mut cell ;

        // match self.get_cell(y_cord, x_cord) {
        //     Some(cell_ref) => {
        //         if GridCellState::Hidden != cell_ref.state { return None }
        //         else { cell = cell_ref; }
        //     },
        //     None => { return None },
        // }

        if GridCellState::Hidden != cell.state { return; }

        cell.state = GridCellState::Visible;

        if let GridCellVariant::WithValue(0) = cell.variant {
            // let set_next_cell_visible = |next_cell| {
            //     self.set_cell_visible(next_cell);
            //     None
            // };

            if cell.y != 0 {
                // let rc_self = Rc::new(self);
                let next_cell = self.get_cell(cell.y - 1, cell.x).unwrap();
                // self.set_cell_visible(next_cell);
            }

            // if cell.y != 0 {
            //     self.get_cell(cell.y - 1, cell.x).and_then(set_next_cell_visible);
            // }
            // if cell.y != self.height {
            //     self.get_cell(cell.y + 1, cell.x).and_then(set_next_cell_visible);
            // }

            // if cell.x != 0 {
            //     self.get_cell(cell.y, cell.x - 1).and_then(set_next_cell_visible);
            // }
            // if cell.x != self.width {
            //     self.get_cell(cell.y, cell.x + 1).and_then(set_next_cell_visible);
            // }
        }

        // None
        // Some(cell)
    }
    
}