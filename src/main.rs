mod app;
mod assets;
mod consts;
mod delegate;
mod game;
mod grid;
mod ui;

use druid::{AppLauncher, PlatformError, WindowDesc};

use app::AppState;
use consts::TITLE;
use delegate::MainDelegate;
use game::Game;

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::build).title(TITLE).resizable(false);
    let state = AppState { game: Game::new() };

    let launcher = AppLauncher::with_window(window).delegate(MainDelegate);
    launcher.launch(state)?;
    Ok(())
}
