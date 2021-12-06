mod grid;
mod game;
mod ui;
mod assets;
mod app;
mod delegate;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc, WidgetExt};
use druid::widget::Flex;

use app::AppState;
use game::Game;
use ui::{InfoPanel, GridWidget};
use delegate::MainDelegate;

fn build_ui() -> impl Widget<AppState> {
    let mut flex = Flex::column();
    flex.add_child(InfoPanel::new());
    flex.add_child(GridWidget::new());
    flex.center()
}

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_ui ).title("Saperke");
    let state = AppState { game: Game::new() };

    AppLauncher::with_window(window)
        .delegate(MainDelegate)
        .launch(state)?;

    Ok(())
}
