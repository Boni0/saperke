mod state;

use crate::grid::Grid;
// use std::{sync};
// use sync::{Arc, Mutex};
use druid::{Data, Lens};

pub use state::{
    GameState,
    GameEndState
};

#[derive(Clone, Data, Lens)]
pub struct Game {
    pub grid: Grid,
    pub mines_count: usize,
    // pub timer_sec: Arc<Mutex<usize>>,
    pub state: GameState
}

impl Game {
    pub fn new() -> Game {
        let test_width: usize = 10;
        let test_height: usize = 10;
        let test_mines: usize = 7;

        let mut grid = Grid::new_rectangle_or_square_grid(test_height, test_width);
        grid.set_mines_to_cells_randomly(test_mines);

        Game {
            grid,
            mines_count: test_mines,
            // timer_sec: Arc::new(Mutex::new(0)),
            // state: GameState::NotStarted
            state: GameState::Running
        }
    }
}