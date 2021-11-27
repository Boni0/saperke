mod controllers;

use std::str::FromStr;

use druid::{LensExt, Widget, WidgetExt, lens};
use druid::widget::{List, Painter, Svg, SvgData};

use crate::assets::{
    EMPTY_SVG_BG, 
    NUMS_SVG_BG_ARRAY, 
    TILE_OPENED_SVG_BG, 
    TILE_UNOPENED_SVG_BG,
    BOMB_SIGN_SVG_BG,
    FLAG_SIGN_SVG_BG,
    QUESTION_MARK_SIGN_SVG_BG
};
use crate::app::AppState;
use crate::game::Game;
use crate::grid::{GridCell, GridCellState, Grid, GridCellVariant};

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
            let cell_bg = SvgData::from_str(
                match cell.state {
                    GridCellState::Active | GridCellState::Visible => TILE_OPENED_SVG_BG,
                    _ => TILE_UNOPENED_SVG_BG,
                }
            )
            .unwrap_or(SvgData::empty());

            Svg::new(cell_bg).paint(ctx, cell, env);

            let cell_value = SvgData::from_str(match cell.state {
                GridCellState::Tagged => FLAG_SIGN_SVG_BG,
                GridCellState::Questioned => QUESTION_MARK_SIGN_SVG_BG,
                GridCellState::Visible => {
                    match cell.variant {
                        GridCellVariant::WithValue(value) => NUMS_SVG_BG_ARRAY[value as usize],
                        GridCellVariant::WithBomb => BOMB_SIGN_SVG_BG,
                        GridCellVariant::NonExist => EMPTY_SVG_BG,
                    }
                },
                _ => EMPTY_SVG_BG
            })
            .unwrap_or(SvgData::empty());

            Svg::new(cell_value).paint(ctx, cell, env);
        }); 

        Svg::new(SvgData::empty())
            .fix_size(23.0, 23.0)
            .background(brush)
            .controller(GridCellController)
    }
}