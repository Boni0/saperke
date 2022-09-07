mod app;
mod assets;
mod consts;
mod delegate;
mod game;
mod grid;
mod menu;
mod ui;
mod variants;

use druid::{AppLauncher, PlatformError, Size, WindowDesc};

use app::AppState;
use consts::TITLE;
use delegate::MainDelegate;
use game::Game;
use grid::{GridConfig, GridPredefinedBoxDifficulty};

fn main() -> Result<(), PlatformError> {
    let state = AppState {
        game: Game::new(GridConfig::predefined_box(
            GridPredefinedBoxDifficulty::Beginner,
        )),
    };

    let window = WindowDesc::new(ui::main_window_build())
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
        .menu(menu::create_app_menu);

    let app_window_id = window.id;
    let launcher = AppLauncher::with_window(window).delegate(MainDelegate::new(app_window_id));
    launcher.launch(state)?;
    Ok(())
}
