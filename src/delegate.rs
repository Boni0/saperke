use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Target, Selector};

use crate::app::AppState;
use crate::game::{GameState, GameEndState};
use crate::grid::{GridCellPoint, GridCellState, GridCellVariant};

pub const CHANGE_GRID_CELL_STATE: Selector<(GridCellPoint, GridCellState)> = Selector::new("grid.state.set");
pub const SET_GRID_CELLS_VISIBLE: Selector<GridCellPoint> = Selector::new("grid.visible.set");

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
                    .get_cell_mut(point.clone())
                    .and_then(|cell| {
                        cell.state = new_state.clone();
                        Some(())
                    });

                return Handled::Yes
            }

            if let Some(point) = cmd.get(SET_GRID_CELLS_VISIBLE) {
                state
                    .game
                    .grid
                    .set_cell_visible(point.clone())
                    .and_then(|variant| {
                        if variant == GridCellVariant::WithBomb {
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