mod controllers;

use std::str::FromStr;

use druid::{LensExt, Widget, WidgetExt, lens};
use druid::widget::{List, Painter, Svg, SvgData, Either};

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
use crate::grid::{GridCell, GridCells, GridCellState, GridCellValue, Grid, GridCellVariant};

use controllers::GridCellController;

pub struct GridWidget;

impl GridWidget {
    pub fn new() -> impl Widget<AppState> {
        List::new(|| {
            List::new(|| {
                GridWidget::cell()
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

    fn cell() -> impl Widget<GridCell> {
        let brush = Painter::new(move |ctx, cell: &GridCell, env| {
            if let GridCellVariant::Exist(cell_data) = &cell.variant {

                let cell_bg = SvgData::from_str(
                    if cell_data.is_visible || cell_data.state == GridCellState::Active {
                        TILE_OPENED_SVG_BG
                    } else {
                        TILE_UNOPENED_SVG_BG
                    }
                )
                .unwrap_or(SvgData::empty());
    
                Svg::new(cell_bg).paint(ctx, cell, env);
    
                let cell_value = SvgData::from_str(match cell_data.state {
                    GridCellState::Tagged => FLAG_SIGN_SVG_BG,
                    GridCellState::Questioned => QUESTION_MARK_SIGN_SVG_BG,
                    _ => {
                        if cell_data.is_visible {
                            match cell_data.value {
                                GridCellValue::Number(value) => NUMS_SVG_BG_ARRAY[value as usize],
                                GridCellValue::Bomb => BOMB_SIGN_SVG_BG,
                            }
                        }
                        else {
                            EMPTY_SVG_BG
                        }
                    }
                })
                .unwrap_or(SvgData::empty());
    
                Svg::new(cell_value).paint(ctx, cell, env);

            }
        }); 

        Either::new(
            |cell: &GridCell, _| cell.variant != GridCellVariant::NonExist, 
            
            Svg::new(SvgData::empty())
                .fix_size(23.0, 23.0)
                .background(brush)
                .controller(GridCellController), 
            
            Svg::new(SvgData::empty())
                .fix_size(23.0, 23.0)
        )
    }
}