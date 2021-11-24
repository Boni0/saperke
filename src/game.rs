use crate::grid::GridStruct;
use std::{sync};
use sync::{Arc, Mutex};
use druid::{Data, Lens};

#[derive(Clone, PartialEq, Data)]
pub enum GameEndState {
    Loss,
    Win
}

#[derive(Clone, Data)]
pub enum GameState {
    NotStarted,
    Started,
    Paused,
    EndState(GameEndState)
}

#[derive(Clone, Data, Lens)]
pub struct Game {
    pub grid: GridStruct,
    pub mines_count: usize,
    pub timer_sec: Arc<Mutex<usize>>,
    pub state: GameState
}

impl Game {
    pub fn new() -> Game {
        let test_width: usize = 10;
        let test_height: usize = 10;
        let test_mines: usize = 7;

        let mut grid = GridStruct::new_rectangle_or_square_grid(test_height, test_width);
        grid.set_mines_to_cells_randomly(test_mines);

        Game {
            grid,
            mines_count: test_mines,
            timer_sec: Arc::new(Mutex::new(0)),
            // state: GameState::NotStarted
            state: GameState::Started
        }
    }

    // fn spawn_timer_thread(&mut self, timer_start: usize) -> JoinHandle<()> {
    //     let timer_arc = self.timer_sec.clone();
    //     thread::spawn(move || {
    //         let mut count: usize = timer_start;
    //         loop {
    //             thread::sleep(time::Duration::new(1,0));
    //             count += 1;
    //             *timer_arc.lock().unwrap() = count;
    //         }
    //     })
    // }

    // pub fn start(&mut self) {
    //     let timer_arc = self.timer_sec.clone();
    //     let a = thread::spawn(move || {
    //         let mut count: usize = 0;
    //         loop {
    //             thread::sleep(time::Duration::new(1,0));
    //             count += 1;
    //             *timer_arc.lock().unwrap() = count;
    //         }
    //     });

    //     self.state = GameState::Started;
    // }
}