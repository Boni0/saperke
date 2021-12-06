use druid::{Widget, WidgetExt, lens};
use druid::widget::Button;

use crate::app::AppState;
use crate::game::{Game, GameState};

pub struct InfoPanel;

impl InfoPanel {
    pub fn new() -> impl Widget<AppState> {
        Button::new("New")
            .on_click(|_, game: &mut Game, _| {
                game.grid.refresh();
                game.state = GameState::Running;
            })
            .lens(lens!(AppState, game))
    }
}