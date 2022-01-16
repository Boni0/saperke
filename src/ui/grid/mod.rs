mod controllers;
mod cell;

use druid::{LensExt, Widget, WidgetExt, lens};
use druid::widget::List;

use crate::app::AppState;
use crate::game::Game;
use crate::grid::{GridCells, Grid};

use controllers::GridCellController;
use cell::CellWidget;

pub struct GridWidget;

impl GridWidget {
    pub fn new() -> impl Widget<AppState> {
        List::new(|| {
            List::new(|| {
                CellWidget::new()
            })
            .horizontal()
        })
        .lens(
            lens!(AppState, game)
                .then(lens!(Game, grid))
                .then(lens!(Grid, cells))
                .then(lens!(GridCells, matrix))
        )
    }
}