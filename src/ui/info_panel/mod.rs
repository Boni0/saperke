use std::convert::TryInto;

use druid::{Widget, WidgetExt, lens, LensExt};
use druid::widget::{Button, Flex};

use crate::app::AppState;
use crate::game::Game;
use crate::ui::{ThreeColumnCounter, TimerCounter};

pub struct InfoPanel;

impl InfoPanel {
    pub fn new() -> impl Widget<AppState> {
        let mut flex = Flex::row();

        flex.add_child(
            ThreeColumnCounter::new()
                .lens(lens::Identity.map(
                |state: &AppState| {
                    let bombs_count: i64 = state.game.grid.bombs.count.try_into().unwrap();
                    let tagged_count: i64 = state.game.grid.cells.tagged_points.len().try_into().unwrap();
                    bombs_count - tagged_count
                },
                |_, _| {}
                ))
        );

        flex.add_child(
            Button::new("New")
            .on_click(|_, game: &mut Game, _| game.restart())
            .lens(lens!(AppState, game))
        );

        flex.add_child(
            TimerCounter::new()
        );

        flex
    }
}