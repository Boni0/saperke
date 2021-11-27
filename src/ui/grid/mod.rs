mod controllers;

use std::str::FromStr;

use druid::{LensExt, Widget, WidgetExt, lens};
use druid::widget::{List, Painter, Svg, SvgData};

use crate::assets::{TILE_OPENED_BG, TILE_UNOPENED_BG};
use crate::app::AppState;
use crate::game::Game;
use crate::grid::{GridCell, GridCellState, Grid};

use controllers::GridCellController;

pub struct GridWidget;

impl GridWidget {
    pub fn new() -> impl Widget<AppState> {
        List::new(|| {
            List::new(|| {
                GridWidget::create_cell_test()
            })
            .horizontal()
        })
        .lens(
            lens!(AppState, game)
                .then(lens!(Game, grid))
                .then(lens!(Grid, cells))
        )
    }

    fn create_cell_test() -> impl Widget<GridCell> {
        let brush = Painter::new(move |ctx, cell: &GridCell, env| {
            Svg::new(
                SvgData::from_str(
                    match cell.state {
                        GridCellState::Active | GridCellState::Visible => TILE_OPENED_BG,
                        _ => TILE_UNOPENED_BG,
                    }
                )
                .unwrap_or(SvgData::empty())
            ).paint(ctx, cell, env);
        }); 

        Svg::new(SvgData::default())
            .fix_size(23.0, 23.0)
            .background(brush)
            .controller(GridCellController)
    }
}