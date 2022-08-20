use druid::{widget::Either, Widget, WidgetExt};

use super::utils;
use crate::grid::{GridCell, GridCellVariant};

pub struct CellWidget;

impl CellWidget {
    pub fn new() -> impl Widget<GridCell> {
        Either::<GridCell>::new(
            |cell: &GridCell, _| cell.variant == GridCellVariant::NonExist,
            utils::create_cell_svg(),
            utils::create_cell_svg().background(utils::get_cell_painter()),
        )
    }
}
