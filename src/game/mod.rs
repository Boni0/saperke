mod state;
mod time;

use crate::grid::{Grid, GridBombsConfig, GridCellPoint, GridCellValue, GridShape, GridSize};
use druid::{Data, Lens};

pub use state::{GameEndState, GameState};

use self::time::GameTime;

#[derive(Clone, Data, Lens)]
pub struct Game {
    pub grid: Grid,
    pub state: GameState,
    pub time: GameTime,
}

impl Game {
    pub fn new() -> Game {
        let test_width: usize = 24;
        let test_height: usize = 24;

        let grid = Grid::new(
            GridSize {
                width: test_width,
                height: test_height,
            },
            GridShape::RectangleOrSquare,
            &GridBombsConfig::Randomized(10),
        );

        Game {
            grid,
            state: GameState::NotStarted,
            time: GameTime::new(),
        }
    }

    pub fn restart(&mut self) {
        self.grid.refresh();
        self.state = GameState::NotStarted;
        self.time.reset();
    }

    pub fn handle_cell_open(&mut self, point: &GridCellPoint) {
        if self.state == GameState::NotStarted {
            self.state = GameState::Running;
        }

        self.grid
            .handle_cells_visible(point)
            .and_then(|value_of_first_visible_cell| {
                let mut end_state_option = None;

                match value_of_first_visible_cell {
                    GridCellValue::Number(_) => {
                        if self.grid.cells.visible_count
                            == (self.grid.cells.exist_count - self.grid.bombs.count)
                        {
                            end_state_option = Some(GameEndState::Win);
                        }
                    }
                    GridCellValue::Bomb => {
                        self.grid.set_all_bombs_visible();
                        self.grid.set_all_flagged_cells_to_verify();

                        end_state_option = Some(GameEndState::Loss);
                    }
                }

                end_state_option
            })
            .and_then(|end_state| {
                self.state = GameState::EndState(end_state);
                Some(())
            });
    }
}
