mod cell;
mod controllers;
mod utils;

use crate::grid::{Grid, GridCells};
use druid::widget::{Container, List};
use druid::{lens, LensExt, Widget, WidgetExt};

use cell::CellWidget;
use controllers::GridController;

pub struct GridWidget;

impl GridWidget {
    pub fn new() -> impl Widget<Grid> {
        Container::new(
            List::new(|| List::new(|| CellWidget::new()).horizontal())
                .lens(lens!(Grid, cells).then(lens!(GridCells, matrix))),
        )
        .controller(GridController::new())
    }
}
