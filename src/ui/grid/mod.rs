mod cell;
mod controllers;
mod utils;

use druid::widget::List;
use druid::{lens, LensExt, Widget, WidgetExt};

use crate::grid::{Grid, GridCells};

use cell::CellWidget;
use controllers::GridCellController;

use super::border_box::{BorderBox, BorderColorPattern};

pub struct GridWidget;

impl GridWidget {
    pub fn new() -> impl Widget<Grid> {
        BorderBox::new(
            List::new(|| List::new(|| CellWidget::new()).horizontal())
                .lens(lens!(Grid, cells).then(lens!(GridCells, matrix))),
            BorderColorPattern::DarkerFirst,
        )
    }
}
