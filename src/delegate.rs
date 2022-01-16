use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, Selector};

use crate::app::AppState;
use crate::game::{GameState};
use crate::grid::{GridCellPoint, GridCellState, GridCellFlaggedState};
use crate::consts::TIMER_INTERVAL;

pub const HANDLE_CELL_OPEN: Selector<GridCellPoint> = Selector::new("HANDLE_CELL_OPEN");
pub const HANDLE_CELL_TOGGLE_HOVER: Selector<(GridCellPoint, GridCellState)> = Selector::new("HANDLE_CELL_TOGGLE_HOVER");
pub const HANDLE_CELL_FLAGGING: Selector<(GridCellPoint, Option<GridCellFlaggedState>)> = Selector::new("HANDLE_CELL_FLAGGING");
pub const HANDLE_TIMER: Selector = Selector::new("HANDLE_TIMER");

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
        if 
            state.game.state == GameState::NotStarted
            || state.game.state == GameState::Running 
        {
            if let Some(point) = cmd.get(HANDLE_CELL_OPEN) {
                state.game.handle_cell_open(point);
                return Handled::Yes
            }

            if let Some((point, new_state)) = cmd.get(HANDLE_CELL_TOGGLE_HOVER) {
                if let Some(cell_data) = state.game.grid.cells.get_existing_cell(point) {
                    if !cell_data.is_visible {
                        cell_data.state = new_state.clone();
                    }
                }
                return Handled::Yes
            }

            if let Some((point, option_flagged_state)) = cmd.get(HANDLE_CELL_FLAGGING) {
                state.game.grid.handle_cell_flagged_state(point, option_flagged_state.clone());
                return Handled::Yes
            }
        }

        if let Some(_) = cmd.get(HANDLE_TIMER) {
            if state.game.state == GameState::Running {
                state.game.time += TIMER_INTERVAL;
                return Handled::Yes
            }
        }

        Handled::No
    }

}