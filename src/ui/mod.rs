mod border_box;
mod game_container;
mod grid;
mod info_panel;
mod three_column_counter;
mod window_size_observer;

use crate::consts::{BACKGROUND_COLOR_HEX, FLEX_COMMON_SPACING_SIZE};
use crate::game::Game;
use crate::grid::Grid;
use crate::AppState;
use druid::widget::{Flex, LensWrap};
use druid::{lens, Color, LensExt, Widget, WidgetExt};

pub use grid::GridWidget;
pub use info_panel::InfoPanel;
pub use three_column_counter::ThreeColumnCounter;
pub use window_size_observer::WindowSizeObserverWidget;

use self::border_box::{BorderBox, BorderColorPattern};
use self::game_container::GameContainerWidget;

pub fn build() -> impl Widget<AppState> {
    let mut flex = Flex::column();

    // Invisible Window Size observer
    flex.add_child(LensWrap::new(
        WindowSizeObserverWidget,
        lens!(AppState, game)
            .then(lens!(Game, grid))
            .then(lens!(Grid, size)),
    ));

    flex.add_child(InfoPanel::new());
    flex.add_spacer(FLEX_COMMON_SPACING_SIZE);

    flex.add_child(LensWrap::new(
        GameContainerWidget::new(),
        lens!(AppState, game),
    ));
    flex.add_spacer(FLEX_COMMON_SPACING_SIZE);

    BorderBox::new(
        flex.padding(FLEX_COMMON_SPACING_SIZE).center(),
        BorderColorPattern::LigherFirst,
    )
    .background(Color::from_hex_str(BACKGROUND_COLOR_HEX).unwrap())
}
