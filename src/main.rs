mod grid;
mod game;
mod ui;
mod assets;
mod app;
mod delegate;
mod consts;

use std::thread;
use druid::{AppLauncher, PlatformError, WindowDesc, ExtEventSink, Target};

use app::AppState;
use game::Game;
use delegate::{MainDelegate, HANDLE_TIMER};
use consts::{TITLE, TIMER_INTERVAL};

fn create_timer(event_sink: ExtEventSink) {
    loop {
        event_sink.submit_command(HANDLE_TIMER, (), Target::Auto).unwrap();
        thread::sleep(TIMER_INTERVAL);
    }
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::build ).title(TITLE);
    let state = AppState { game: Game::new() };

    let launcher = AppLauncher::with_window(window).delegate(MainDelegate);
    // let event_sink = launcher.get_external_handle();
    // Temp remove
    //thread::spawn(move || create_timer(event_sink));

    launcher.launch(state)?;
    Ok(())
}
