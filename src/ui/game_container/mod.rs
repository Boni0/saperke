mod controllers;

use druid::widget::LensWrap;
use druid::{lens, Widget, WidgetExt};

use controllers::GameContainerController;

use crate::game::Game;

use super::border_box::{BorderBox, BorderColorPattern};
use super::GridWidget;

pub struct GameContainerWidget;

impl GameContainerWidget {
    pub fn new() -> impl Widget<Game> {
        BorderBox::new(
            LensWrap::new(GridWidget::new(), lens!(Game, grid)),
            BorderColorPattern::DarkerFirst,
        )
        .controller(GameContainerController)
    }
}
