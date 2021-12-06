use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, Selector};

use crate::app::AppState;
use crate::game::{GameState};
use crate::grid::{GridCellPoint, GridCellFlaggedState};


pub const GRID_OPEN_CELL: Selector<GridCellPoint> = Selector::new("grid.open_cell");
pub const GRID_SET_CELL_FLAGGED_STATE: Selector<(GridCellPoint, Option<GridCellFlaggedState>)> = Selector::new("grid.set_cell_flagged_state");

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
        if let GameState::EndState(_) = state.game.state {
            if let Some((point, option_flagged_state)) = cmd.get(GRID_SET_CELL_FLAGGED_STATE) {
                state.game.grid.set_cell_flagged_state(point, option_flagged_state.clone());
                return Handled::Yes
            }

            if let Some(point) = cmd.get(GRID_OPEN_CELL) {
                state.game.open_cell(point);
                return Handled::Yes
            }
        }

        Handled::No
    }
}