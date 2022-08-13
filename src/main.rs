mod app;
mod assets;
mod consts;
mod custom_rectangle_square;
mod delegate;
mod game;
mod grid;
mod menu;
mod ui;
mod unusual_predefined;

use druid::{AppLauncher, PlatformError, Selector, Size, WindowDesc};

use app::AppState;
use consts::TITLE;
use delegate::MainDelegate;
use game::{Game, GameDifficultyGrid, StandardGameDifficulty};

pub const EMPTY: Selector<()> = Selector::new("EMPTY");

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::main_window_build)
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
        .menu(menu::create_app_menu());

    let state = AppState {
        game: Game::new(GameDifficultyGrid::Standard(
            StandardGameDifficulty::Beginner,
        )),
    };

    let app_window_id = window.id;
    let launcher = AppLauncher::with_window(window).delegate(MainDelegate::new(app_window_id));
    launcher.launch(state)?;
    Ok(())
}
