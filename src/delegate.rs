use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::app::AppState;
use crate::game::{Game, GameDifficultyGrid, GameState, StandardGameDifficulty};
use crate::grid::GridCellPoint;

pub const CELL_ACTIVE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_ACTIVE_BY_MULTIPLE_POINTS");

pub const CELL_IDLE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_IDLE_BY_MULTIPLE_POINTS");

pub const CELL_TOGGLE_FLAG_BY_POINT: Selector<GridCellPoint> =
    Selector::new("CELL_TOGGLE_FLAG_BY_POINT");
pub const CELL_OPEN_BY_POINT: Selector<GridCellPoint> = Selector::new("CELL_OPEN_BY_POINT");

pub const RESTART_GAME: Selector = Selector::new("NEW_GAME");
pub const NEW_GAME_STANDARD: Selector<StandardGameDifficulty> = Selector::new("NEW_GAME_STANDARD");

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
                if game_clone.state == GameState::NotStarted {
                    game_clone.state = GameState::Running
                }

                game_clone.handle_cell_open(point);
                delegate_handled = Handled::Yes;
            }

            if delegate_handled == Handled::Yes {
                state.game = game_clone;
            }
        }

        // if cmd.is(RESTART_GAME) {
        //     state.game.restart();
        //     delegate_handled = Handled::Yes;
        // }

        // if let Some(standard_difficulty) = cmd.get(NEW_GAME_STANDARD) {
        //     state.game = Game::new(GameDifficultyGrid::Standard(standard_difficulty.clone()));
        //     delegate_handled = Handled::Yes;
        // }

        delegate_handled
    }
}
