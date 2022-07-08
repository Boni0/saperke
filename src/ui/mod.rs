mod grid;
mod info_panel;
mod three_column_counter;
mod window_size_observer;

use crate::game::Game;
use crate::grid::Grid;
use crate::AppState;
use druid::widget::{Flex, LensWrap};
use druid::{lens, LensExt, Widget, WidgetExt};

pub use grid::GridWidget;
pub use info_panel::InfoPanel;
pub use three_column_counter::ThreeColumnCounter;
pub use window_size_observer::WindowSizeObserverWidget;

pub fn build() -> impl Widget<AppState> {
    let mut flex = Flex::column();

    flex.add_child(InfoPanel::new());
    flex.add_child(LensWrap::new(
        GridWidget::new(),
        lens!(AppState, game).then(lens!(Game, grid)),
    ));

    flex.add_child(LensWrap::new(
        WindowSizeObserverWidget,
        lens!(AppState, game)
            .then(lens!(Game, grid))
            .then(lens!(Grid, size)),
    ));
    flex.center()
}
