mod border_box;
mod game_container;
mod grid;
mod info_panel;
mod three_column_counter;
mod window_size_observer;

use crate::consts::BACKGROUND_COLOR_HEX;
use crate::game::Game;
use crate::grid::Grid;
use crate::AppState;
use druid::widget::{Container, Flex, LensWrap};
use druid::{lens, Color, LensExt, Widget, WidgetExt};

pub use grid::GridWidget;
pub use info_panel::InfoPanel;
pub use three_column_counter::ThreeColumnCounter;
pub use window_size_observer::WindowSizeObserverWidget;

use self::border_box::{BorderBox, BorderColorPattern};
use self::game_container::GameContainerWidget;

pub fn build() -> impl Widget<AppState> {
    let mut flex = Flex::column();

    flex.add_child(InfoPanel::new());

    // flex.add_child(LensWrap::new(
    //     GridWidget::new(),
    //     lens!(AppState, game).then(lens!(Game, grid)),
    // ));

    flex.add_child(LensWrap::new(
        GameContainerWidget::new(),
        lens!(AppState, game),
    ));

    flex.add_child(LensWrap::new(
        WindowSizeObserverWidget,
        lens!(AppState, game)
            .then(lens!(Game, grid))
            .then(lens!(Grid, size)),
    ));

    BorderBox::new(flex.center(), BorderColorPattern::LigherFirst)
        .background(Color::from_hex_str(BACKGROUND_COLOR_HEX).unwrap())
}
