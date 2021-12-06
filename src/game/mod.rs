mod state;

use druid::im::Vector;

use crate::grid::{
    Grid,
    GridSize,
    GridShape,
    GridCellValue,
    GridBombsConfig,
    GridCellPoint, 
    GridCellState,
    GridCellOpenedState
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
        // let test_mines: usize = 1;

        let mut non_existed_test = Vector::new();
        non_existed_test.push_back(GridCellPoint {
            x: 5,
            y: 5
        });

        let grid = Grid::new(
            GridSize {
                width: test_width,
                height: test_height
            },
            GridShape::RectangleOrSquare,
            // GridShape::Unusual(non_existed_test),
            // &GridBombsConfig::Randomized(test_mines)
            &GridBombsConfig::Randomized(10)
        );

        Game {
            grid,
            state: GameState::Running
        }
    }

    pub fn open_cell(&mut self, point: &GridCellPoint) {
        self
            .grid
            .set_idle_cells_visible(point)
            .and_then(|value_of_first_visible_cell| {
                match value_of_first_visible_cell {
                    GridCellValue::Number(_) => {
                        if self.check_game_win_state() { Some(GameEndState::Win) } 
                        else { None }
                    },
                    GridCellValue::Bomb => {
                        if let Some(cell_data) = self.grid.get_existing_cell_mut(point) {
                            cell_data.state = GridCellState::Opened(GridCellOpenedState::CausedLoss);
                        }

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

    fn check_game_win_state(&self) -> bool {
        self.grid.cells.visible_count == (self.grid.cells.exist_count - self.grid.bombs.count)
    }
}