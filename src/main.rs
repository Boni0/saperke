mod app;
mod assets;
mod consts;
mod delegate;
mod game;
mod grid;
mod menu;
mod ui;
mod variants;

use std::env;

use druid::{AppLauncher, PlatformError, Size, WindowDesc};

use app::AppState;
use consts::{GAME_BEGINNER_DIFFICULTY_SIZE, TITLE};
use delegate::MainDelegate;
use game::Game;
use grid::{GridConfig, GridPredefinedBoxDifficulty};
use ui::WindowSizeObserverWidget;

fn main() -> Result<(), PlatformError> {
    let state = AppState {
        game: Game::new(GridConfig::predefined_box(
            GridPredefinedBoxDifficulty::Beginner,
        )),
    };

    let mut window = WindowDesc::new(ui::main_window_build())
        .title(TITLE)
        .resizable(false)
        .window_size(Size {
            width: 10.0,
            height: 10.0,
        })
        .with_min_size(Size {
            width: 10.0,
            height: 10.0,
        })
        .with_min_size(Size {
            width: 10.0,
            height: 10.0,
        })
        .menu(menu::create_app_menu);

    if env::consts::OS == "windows" {
        window = window.with_min_size(WindowSizeObserverWidget::get_window_size(
            &GAME_BEGINNER_DIFFICULTY_SIZE,
        ))
    }

    let app_window_id = window.id;
    let launcher = AppLauncher::with_window(window).delegate(MainDelegate::new(app_window_id));
    launcher.launch(state)?;
    Ok(())
}
