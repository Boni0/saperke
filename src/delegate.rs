use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, Selector};

use crate::app::AppState;
use crate::game::{GameState, GameEndState};
use crate::grid::{GridCellPoint, GridCellValue, GridCellState};

pub const GRID_SET_CELLS_VISIBLE: Selector<GridCellPoint> = Selector::new("grid.set_cells_visible");
pub const GRID_SET_CELL_STATE: Selector<(GridCellPoint, GridCellState)> = Selector::new("grid.set_cell_state");

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
            if let Some((point, new_state)) = cmd.get(GRID_SET_CELL_STATE) {
                state
                    .game
                    .grid
                    .set_cell_state(point, new_state.clone());

                return Handled::Yes
            }

            if let Some(point) = cmd.get(GRID_SET_CELLS_VISIBLE) {
                state
                    .game
                    .grid
                    .set_cells_visible(point)
                    .and_then(|value| {
                        if let GridCellValue::Bomb = value {
                            state.game.state = GameState::EndState(GameEndState::Loss);
                        }

                        Some(())
                    });

                return Handled::Yes
            }
        }

        Handled::No
    }
}