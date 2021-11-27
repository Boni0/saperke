use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, Selector};

use crate::app::AppState;
use crate::game::GameState;
use crate::grid::{GridCellPoint, GridCellState};

pub const CHANGE_GRID_CELL_STATE: Selector<(GridCellPoint, GridCellState)> = Selector::new("grid.active.set");
// pub const SET_GRID_CELL_TAGGED: Selector = Selector::new("grid.tagged.set");
// pub const SET_GRID_CELLS_VISIBLE: Selector = Selector::new("grid.visible.set");

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
        if state.game.state == GameState::Running {
            if let Some((point, new_state)) = cmd.get(CHANGE_GRID_CELL_STATE) {
                state
                    .game
                    .grid
                    .get_cell_mut(point.y, point.x)
                    .and_then(|cell| {
                        cell.state = new_state.clone();
                        Some(())
                    });

                return Handled::Yes
            }
        }

        Handled::No
    }
}