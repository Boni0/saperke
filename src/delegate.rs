use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::app::AppState;
use crate::consts::TIMER_INTERVAL;
use crate::game::{Game, GameState};
use crate::grid::{
    Grid, GridCell, GridCellFlaggedState, GridCellPoint, GridCellState, GridExistingCell,
};

// pub const HANDLE_CELL_OPEN: Selector<GridCellPoint> = Selector::new("HANDLE_CELL_OPEN");
// pub const HANDLE_CELL_TOGGLE_HOVER: Selector<(GridCellPoint, GridCellState)> =
//     Selector::new("HANDLE_CELL_TOGGLE_HOVER");
// pub const HANDLE_CELL_FLAGGING: Selector<(GridCellPoint, Option<GridCellFlaggedState>)> =
//     Selector::new("HANDLE_CELL_FLAGGING");
pub const HANDLE_TIMER: Selector = Selector::new("HANDLE_TIMER");
// pub const HANDLE_ACTIVE_GRID: Selector<bool> = Selector::new("HANDLE_ACTIVE_GRID");

// New delagate for grid
pub const CELL_ACTIVE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_ACTIVE_BY_MULTIPLE_POINTS");

pub const CELL_IDLE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_IDLE_BY_MULTIPLE_POINTS");

pub const CELL_TOGGLE_FLAG_BY_POINT: Selector<GridCellPoint> =
    Selector::new("CELL_TOGGLE_FLAG_BY_POINT");
pub const CELL_OPEN_BY_POINT: Selector<GridCellPoint> = Selector::new("CELL_OPEN_BY_POINT");

pub struct MainDelegate;

impl AppDelegate<AppState> for MainDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        state: &mut AppState,
        _env: &Env,
    ) -> Handled {
        let mut delegate_handled = Handled::No;

        if state.game.state == GameState::NotStarted || state.game.state == GameState::Running {
            let mut game_clone: Game = state.game.clone();

            if let Some(point_vec) = cmd.get(CELL_IDLE_BY_MULTIPLE_POINTS) {
                for point in point_vec.into_iter() {
                    game_clone.grid.cells.set_cell_idle_state(point);
                }
                delegate_handled = Handled::Yes;
            }

            if let Some(point_vec) = cmd.get(CELL_ACTIVE_BY_MULTIPLE_POINTS) {
                for point in point_vec.into_iter() {
                    game_clone.grid.cells.set_cell_active_state(point);
                }
                delegate_handled = Handled::Yes;
            }

            if let Some(point) = cmd.get(CELL_TOGGLE_FLAG_BY_POINT) {
                game_clone.grid.cells.toggle_cell_flagged_state(point);
                delegate_handled = Handled::Yes;
            }

            if let Some(point) = cmd.get(CELL_OPEN_BY_POINT) {
                if state.game.state == GameState::NotStarted {
                    state.game.state = GameState::Running
                }

                game_clone.handle_cell_open(point);
                delegate_handled = Handled::Yes;
            }

            if delegate_handled == Handled::Yes {
                state.game = game_clone;
            }
        }

        // if let Some(_) = cmd.get(HANDLE_TIMER) {
        //     if state.game.state == GameState::Running {
        //         state.game.time += TIMER_INTERVAL;
        //         return Handled::Yes;
        //     }
        // }

        delegate_handled
    }
}
