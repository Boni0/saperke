mod app;
mod assets;
mod consts;
mod delegate;
mod game;
mod grid;
mod ui;

use druid::{AppLauncher, ExtEventSink, PlatformError, Target, WindowDesc};
use std::thread;

use app::AppState;
use consts::{TIMER_INTERVAL, TITLE};
use delegate::{MainDelegate, HANDLE_TIMER};
use game::Game;

#[allow(dead_code)]
fn create_timer(event_sink: ExtEventSink) {
    loop {
        event_sink
            .submit_command(HANDLE_TIMER, (), Target::Auto)
            .unwrap();
        thread::sleep(TIMER_INTERVAL);
    }
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::build).title(TITLE).resizable(false);
    let state = AppState { game: Game::new() };

    let launcher = AppLauncher::with_window(window).delegate(MainDelegate);
    // let event_sink = launcher.get_external_handle();
    // Temp remove
    //thread::spawn(move || create_timer(event_sink));

    launcher.launch(state)?;
    Ok(())
}
