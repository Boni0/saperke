use druid::widget::{Painter, Svg, SvgData};
use druid::{Color, RenderContext, Widget, WidgetExt};
use std::str::FromStr;

use crate::assets::{
    BOMB_SIGN_SVG_BG, FLAG_SIGN_SVG_BG, NUMS_SVG_BG_ARRAY, QUESTION_MARK_SIGN_SVG_BG,
    TILE_OPENED_SVG_BG, TILE_UNOPENED_SVG_BG, X_SIGN_SVG_BG,
};
use crate::consts::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH};
use crate::grid::{
    GridCell, GridCellFlaggedState, GridCellOpenedState, GridCellState, GridCellValue,
    GridCellVariant,
};

pub fn create_cell_svg() -> impl Widget<GridCell> {
    Svg::new(SvgData::empty()).fix_size(GRID_CELL_WIDTH, GRID_CELL_HEIGHT)
}

enum PaintInside<'a> {
    Bg(&'a Color),
    Svg(&'a str),
}

pub fn get_cell_painter() -> Painter<GridCell> {
    Painter::new(move |ctx, cell: &GridCell, env| {
        let bounds = ctx.size().to_rect();

        let mut paint = |inside: PaintInside| match inside {
            PaintInside::Bg(color) => ctx.fill(bounds, color),
            PaintInside::Svg(from_str) => {
                if let Ok(svg_data) = SvgData::from_str(from_str) {
                    Svg::new(svg_data).paint(ctx, cell, env);
                }
            }
        };

        if let GridCellVariant::Exist(cell_data) = &cell.variant {
            match cell_data.state {
                GridCellState::Idle => {
                    paint(PaintInside::Svg(TILE_UNOPENED_SVG_BG));
                }
                GridCellState::Active => {
                    paint(PaintInside::Svg(TILE_OPENED_SVG_BG));
                }
                GridCellState::Flagged(GridCellFlaggedState::Questioned) => {
                    paint(PaintInside::Svg(TILE_UNOPENED_SVG_BG));
                    paint(PaintInside::Svg(QUESTION_MARK_SIGN_SVG_BG));
                }
                GridCellState::Flagged(GridCellFlaggedState::Tagged) => {
                    paint(PaintInside::Svg(TILE_UNOPENED_SVG_BG));
                    paint(PaintInside::Svg(FLAG_SIGN_SVG_BG));
                }
                GridCellState::Opened(GridCellOpenedState::NoAction) => {
                    paint(PaintInside::Svg(TILE_OPENED_SVG_BG));
                    if let GridCellValue::Number(value) = cell_data.value {
                        paint(PaintInside::Svg(NUMS_SVG_BG_ARRAY[value as usize]));
                    }
                }
                GridCellState::Opened(GridCellOpenedState::CausedLoss) => {
                    paint(PaintInside::Svg(TILE_OPENED_SVG_BG));
                    paint(PaintInside::Bg(&Color::RED));
                    paint(PaintInside::Svg(BOMB_SIGN_SVG_BG));
                }
                GridCellState::ToVerifyFlag(GridCellFlaggedState::Questioned) => {
                    match cell_data.value {
                        GridCellValue::Number(_) => {
                            paint(PaintInside::Svg(TILE_UNOPENED_SVG_BG));
                            paint(PaintInside::Svg(QUESTION_MARK_SIGN_SVG_BG));
                        }
                        GridCellValue::Bomb => {
                            paint(PaintInside::Svg(TILE_OPENED_SVG_BG));
                            paint(PaintInside::Svg(BOMB_SIGN_SVG_BG));
                        }
                    }
                }
                GridCellState::ToVerifyFlag(GridCellFlaggedState::Tagged) => {
                    match cell_data.value {
                        GridCellValue::Number(_) => {
                            paint(PaintInside::Svg(TILE_OPENED_SVG_BG));
                            paint(PaintInside::Svg(BOMB_SIGN_SVG_BG));
                            paint(PaintInside::Svg(X_SIGN_SVG_BG));
                        }
                        GridCellValue::Bomb => {
                            paint(PaintInside::Svg(TILE_UNOPENED_SVG_BG));
                            paint(PaintInside::Bg(&Color::GREEN));
                            paint(PaintInside::Svg(FLAG_SIGN_SVG_BG));
                        }
                    }
                }
            }
        };
    })
}
