mod state;

use crate::grid::{
    Grid,
    GridSize,
    GridShape,
    GridBombsConfig
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
        let test_mines: usize = 7;

        let grid = Grid::new(
            GridSize {
                width: test_width,
                height: test_height
            },
            GridShape::RectangleOrSquare,
            &GridBombsConfig::Randomized(test_mines)
        );

        Game {
            grid,
            state: GameState::Running
        }
    }
}