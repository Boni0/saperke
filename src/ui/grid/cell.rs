use druid::{
    widget::{Painter, Svg, SvgData},
    Color, RenderContext, Widget, WidgetExt,
};

use super::utils::PainterSvgData;
use crate::{
    consts::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH},
    grid::{
        GridCell, GridCellFlaggedState, GridCellOpenedState, GridCellState, GridCellValue,
        GridCellVariant,
    },
};

enum PaintInside<'a> {
    Bg(&'a Color),
    Svg(&'a SvgData),
}

pub struct CellWidget;

impl CellWidget {
    pub fn new() -> impl Widget<GridCell> {
        Svg::new(SvgData::empty()).fix_size(GRID_CELL_WIDTH, GRID_CELL_HEIGHT)
    }

    pub fn painter(svg_data: PainterSvgData) -> Painter<GridCell> {
        Painter::new(move |ctx, cell: &GridCell, env| {
            let bounds = ctx.size().to_rect();

            let mut paint = |inside: PaintInside| match inside {
                PaintInside::Bg(color) => ctx.fill(bounds, color),
                PaintInside::Svg(svg_data) => {
                    Svg::new(svg_data.clone()).paint(ctx, cell, env);
                }
            };

            if let GridCellVariant::Exist(cell_data) = &cell.variant {
                match cell_data.state {
                    GridCellState::Idle => {
                        paint(PaintInside::Svg(&svg_data.tile_unopened));
                    }
                    GridCellState::Active => {
                        paint(PaintInside::Svg(&svg_data.tile_opened));
                    }
                    GridCellState::Flagged(GridCellFlaggedState::Questioned) => {
                        paint(PaintInside::Svg(&svg_data.tile_unopened));
                        paint(PaintInside::Svg(&svg_data.question_mark));
                    }
                    GridCellState::Flagged(GridCellFlaggedState::Tagged) => {
                        paint(PaintInside::Svg(&svg_data.tile_unopened));
                        paint(PaintInside::Svg(&svg_data.flag_sign));
                    }
                    GridCellState::Opened(GridCellOpenedState::NoAction) => {
                        paint(PaintInside::Svg(&svg_data.tile_opened));
                        if let GridCellValue::Number(value) = cell_data.value {
                            paint(PaintInside::Svg(&svg_data.nums[value as usize]));
                        }
                    }
                    GridCellState::Opened(GridCellOpenedState::CausedLoss) => {
                        paint(PaintInside::Svg(&svg_data.tile_opened));
                        paint(PaintInside::Bg(&Color::RED));
                        paint(PaintInside::Svg(&svg_data.bomb));
                    }
                    GridCellState::ToVerifyFlag(GridCellFlaggedState::Questioned) => {
                        match cell_data.value {
                            GridCellValue::Number(_) => {
                                paint(PaintInside::Svg(&svg_data.tile_unopened));
                                paint(PaintInside::Svg(&svg_data.question_mark));
                            }
                            GridCellValue::Bomb => {
                                paint(PaintInside::Svg(&svg_data.tile_opened));
                                paint(PaintInside::Svg(&svg_data.bomb));
                            }
                        }
                    }
                    GridCellState::ToVerifyFlag(GridCellFlaggedState::Tagged) => {
                        match cell_data.value {
                            GridCellValue::Number(_) => {
                                paint(PaintInside::Svg(&svg_data.tile_opened));
                                paint(PaintInside::Svg(&svg_data.bomb));
                                paint(PaintInside::Svg(&svg_data.x_sign));
                            }
                            GridCellValue::Bomb => {
                                paint(PaintInside::Svg(&svg_data.tile_unopened));
                                paint(PaintInside::Bg(&Color::GREEN));
                                paint(PaintInside::Svg(&svg_data.flag_sign));
                            }
                        }
                    }
                }
            };
        })
    }
}
