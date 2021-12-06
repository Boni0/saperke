mod state;

use crate::grid::{
    Grid,
    GridSize,
    GridShape,
    GridCellValue,
    GridBombsConfig,
    GridCellPoint, 
    GridCellState
};
use druid::{Data, Lens};

pub use state::{
    GameState,
    GameEndState
};

#[derive(Clone, Data, Lens)]
pub struct Game {
    pub grid: Grid,
    pub state: GameState
}

impl Game {
    pub fn new() -> Game {
        let test_width: usize = 10;
        let test_height: usize = 10;

        let grid = Grid::new(
            GridSize {
                width: test_width,
                height: test_height
            },
            GridShape::RectangleOrSquare,
            &GridBombsConfig::Randomized(10)
        );

        Game {
            grid,
            state: GameState::Running
        }
    }

    fn are_all_number_cells_visible(&self) -> bool {
        self.grid.cells.visible_count == (self.grid.cells.exist_count - self.grid.bombs.count)
    }

    pub fn handle_cell_open(&mut self, point: &GridCellPoint) {
        self.grid.cells.set_cell_state(point, GridCellState::Idle);

        self
            .grid
            .handle_cells_visible(point)
            .and_then(|value_of_first_visible_cell| {
                match value_of_first_visible_cell {
                    GridCellValue::Number(_) => {
                        if self.are_all_number_cells_visible() { Some(GameEndState::Win) } 
                        else { None }
                    },
                    GridCellValue::Bomb => {
                        self.grid.set_all_bombs_visible();
                        self.grid.set_all_flagged_cells_to_verify();

                        Some(GameEndState::Loss)
                    },
                }
            })
            .and_then(|end_state| {
                self.state = GameState::EndState(end_state);
                Some(())
            });
    }
    
}