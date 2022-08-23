use druid::commands::QUIT_APP;
use druid::{
    AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Size, Target, WindowDesc, WindowId,
};

use crate::app::AppState;
use crate::consts::{CUSTOM_GAME_SUBTITLE, TITLE};
use crate::game::{Game, GameState};
use crate::grid::{
    GridCellPoint, GridConfig, GridPredefinedBoxDifficulty, GridSize, GridUnusualVariant, SizeUnit,
};
use crate::menu;
use crate::ui::{self, CONFIG_WINDOW_SIZE};

pub const CELL_ACTIVE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_ACTIVE_BY_MULTIPLE_POINTS");

pub const CELL_IDLE_BY_MULTIPLE_POINTS: Selector<Vec<GridCellPoint>> =
    Selector::new("CELL_IDLE_BY_MULTIPLE_POINTS");

pub const CELL_TOGGLE_FLAG_BY_POINT: Selector<GridCellPoint> =
    Selector::new("CELL_TOGGLE_FLAG_BY_POINT");
pub const CELL_OPEN_BY_POINT: Selector<GridCellPoint> = Selector::new("CELL_OPEN_BY_POINT");

pub const RESTART_GAME: Selector = Selector::new("RESTART_GAME");
pub const TOGGLE_PAUSE_GAME: Selector = Selector::new("TOGGLE_PAUSE_GAME");

pub const NEW_GAME_PREDEFINED_BOX: Selector<GridPredefinedBoxDifficulty> =
    Selector::new("NEW_GAME_PREDEFINED_BOX");
pub const NEW_GAME_SIMPLE_UNUSUAL: Selector<(GridUnusualVariant, SizeUnit)> =
    Selector::new("NEW_GAME_SIMPLE_UNUSUAL");
pub const NEW_GAME_SIMPLE_CUSTOM_BOX: Selector<(GridSize, SizeUnit)> =
    Selector::new("NEW_GAME_SIMPLE_CUSTOM_BOX");

pub const OPEN_CUSTOM_GAME_WINDOW: Selector = Selector::new("OPEN_CUSTOM_GAME_WINDOW");

pub struct MainDelegate {
    pub app_window_id: WindowId,
    pub custom_game_window_id: Option<WindowId>,
    pub about_window_id: Option<WindowId>,
}

impl MainDelegate {
    pub fn new(app_window_id: WindowId) -> Self {
        Self {
            app_window_id,
            custom_game_window_id: None,
            about_window_id: None,
        }
    }
}

impl AppDelegate<AppState> for MainDelegate {
    fn window_removed(
        &mut self,
        id: druid::WindowId,
        _data: &mut AppState,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        if id == self.app_window_id {
            ctx.submit_command(QUIT_APP);
        } else if let Some(custom_window_id) = self.custom_game_window_id {
            if id == custom_window_id {
                self.custom_game_window_id = None;
            }
        } else if let Some(about_window_id) = self.about_window_id {
            if id == about_window_id {
                self.about_window_id = None;
            }
        }
    }

    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
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

        if cmd.is(RESTART_GAME) {
            state.game.restart();
            delegate_handled = Handled::Yes;
        }

        if cmd.is(TOGGLE_PAUSE_GAME) {
            match state.game.state {
                GameState::Running => {
                    state.game.state = GameState::Paused;
                }
                GameState::Paused => {
                    state.game.state = GameState::Running;
                }
                _ => (),
            }

            delegate_handled = Handled::Yes;
        }

        if let Some(standard_difficulty) = cmd.get(NEW_GAME_PREDEFINED_BOX) {
            state.game = Game::new(GridConfig::predefined_box(standard_difficulty.clone()));
            delegate_handled = Handled::Yes;
        }

        if let Some((variant, bombs_amount)) = cmd.get(NEW_GAME_SIMPLE_UNUSUAL) {
            state.game = Game::new(GridConfig::simple_unusual(variant.clone(), *bombs_amount));
            delegate_handled = Handled::Yes;
        }

        if let Some((size, bombs_amount)) = cmd.get(NEW_GAME_SIMPLE_CUSTOM_BOX) {
            state.game = Game::new(GridConfig::simple_custom_box(size.clone(), *bombs_amount));
            delegate_handled = Handled::Yes;
        }

        if cmd.is(OPEN_CUSTOM_GAME_WINDOW) && self.custom_game_window_id == None {
            let custom_game_window = WindowDesc::new(ui::custom_game_window_build);
            self.custom_game_window_id = Some(custom_game_window.id);

            ctx.new_window(
                custom_game_window
                    .title(format!("{} - {}", TITLE, CUSTOM_GAME_SUBTITLE))
                    .resizable(false)
                    .with_min_size(Size {
                        width: 0.0,
                        height: 0.0,
                    })
                    .window_size(CONFIG_WINDOW_SIZE),
            );

            delegate_handled = Handled::Yes;
        }

        if delegate_handled == Handled::Yes {
            ctx.set_menu(menu::create_app_menu(&state), self.app_window_id);
        }

        delegate_handled
    }
}
