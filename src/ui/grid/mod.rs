mod controllers;

use std::str::FromStr;

use druid::{LensExt, Widget, WidgetExt, lens, Color, RenderContext};
use druid::widget::{List, Painter, Svg, SvgData, Either};

use crate::assets::{
    NUMS_SVG_BG_ARRAY, 
    TILE_OPENED_SVG_BG, 
    TILE_UNOPENED_SVG_BG,
    BOMB_SIGN_SVG_BG,
    FLAG_SIGN_SVG_BG,
    QUESTION_MARK_SIGN_SVG_BG
};
use crate::app::AppState;
use crate::game::Game;
use crate::grid::{GridCell, GridCells, GridCellState, GridCellFlaggedState, GridCellValue, GridCellOpenedState, Grid, GridCellVariant};

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
                if let Ok(svg_data) = SvgData::from_str(
                    if cell_data.is_visible || cell_data.state == GridCellState::Active {
                        TILE_OPENED_SVG_BG
                    } else {
                        TILE_UNOPENED_SVG_BG
                    }
                ) {
                    Svg::new(svg_data).paint(ctx, cell, env);
                }

                if let Some(color) = match cell_data.state {
                    GridCellState::Opened(GridCellOpenedState::CausedLoss) => Some(&Color::RED),
                    GridCellState::ToVerifyFlag(GridCellFlaggedState::Tagged) => {
                        if let GridCellValue::Bomb = cell_data.value {
                            Some(&Color::GREEN)
                        } else {
                            Some(&Color::RED)
                        }
                    },
                    GridCellState::ToVerifyFlag(GridCellFlaggedState::Questioned) => {
                        if let GridCellValue::Bomb = cell_data.value {
                            Some(&Color::YELLOW)
                        } else {
                            Some(&Color::RED)
                        }
                    },
                    _ => None
                } {
                    let bounds = ctx.size().to_rect();
                    ctx.fill(bounds, color);
                }

                if let Some(asset_str) = match cell_data.state {
                    GridCellState::Flagged(GridCellFlaggedState::Tagged) => Some(FLAG_SIGN_SVG_BG),
                    GridCellState::Flagged(GridCellFlaggedState::Questioned) => Some(QUESTION_MARK_SIGN_SVG_BG),
                    _ => {
                        if cell_data.is_visible {
                            match cell_data.value {
                                GridCellValue::Number(value) => Some(NUMS_SVG_BG_ARRAY[value as usize]),
                                GridCellValue::Bomb => Some(BOMB_SIGN_SVG_BG),
                            }
                        }
                        else {
                            None
                        }
                    }
                } {
                    if let Ok(svg_data) = SvgData::from_str(asset_str) {
                        Svg::new(svg_data).paint(ctx, cell, env);
                    }
                }
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