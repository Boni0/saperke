mod grid;
mod game;
mod ui;
mod assets;
mod app;
mod delegate;

use std::thread;
use std::time::Duration;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc, WidgetExt, ExtEventSink, Target};
use druid::widget::Flex;

use app::AppState;
use game::Game;
use ui::{InfoPanel, GridWidget};
use delegate::{MainDelegate, HANDLE_TIMER};

fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column();
    flex.add_child(InfoPanel::new());
    flex.add_child(GridWidget::new());
    flex.center()
}

fn create_timer(event_sink: ExtEventSink) {
    let duration = Duration::from_millis(10);

    loop {
        event_sink.submit_command(HANDLE_TIMER, (), Target::Auto).unwrap();
        thread::sleep(duration);
    }
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui ).title("Saperke");
    let state = AppState { game: Game::new() };

    let launcher = AppLauncher::with_window(window).delegate(MainDelegate);
    let event_sink = launcher.get_external_handle();

    thread::spawn(move || create_timer(event_sink));

    launcher.launch(state)?;
    Ok(())
}
