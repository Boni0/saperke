use std::str::FromStr;

use druid::{Widget, WidgetExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Color, RenderContext};
use druid::widget::{Svg, SvgData, Painter};

use crate::assets::{TILE_OPENED_SVG_BG, TILE_UNOPENED_SVG_BG, FLAG_SIGN_SVG_BG, QUESTION_MARK_SIGN_SVG_BG, NUMS_SVG_BG_ARRAY, BOMB_SIGN_SVG_BG};
use crate::grid::{GridCell, GridCellState, GridCellFlaggedState, GridCellValue, GridCellOpenedState, GridCellVariant};

use super::GridCellController;

pub struct CellWidget {
    cell_widget: Box<dyn Widget<GridCell>>,
}

impl CellWidget {
    pub fn new() -> Self {
        Self {
            cell_widget: Box::new(
                Svg::new(SvgData::empty()).fix_size(23.0, 23.0)
           ),
        }
    }

    fn get_painter(&self) -> Painter<GridCell> {
        Painter::new(move |ctx, cell: &GridCell, env| {
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
        })
    }
}

impl Widget<GridCell> for CellWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut GridCell, env: &Env) {
        self.cell_widget.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &GridCell, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                if data.variant != GridCellVariant::NonExist {
                    self.cell_widget = Box::new(
                        Svg::new(SvgData::empty())
                        .fix_size(23.0, 23.0)
                        .background(self.get_painter())
                        .controller(GridCellController)
                    )
                }
            },
            _ => {}
        }

        self.cell_widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &GridCell, data: &GridCell, env: &Env) {
        self.cell_widget.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &GridCell, env: &Env) -> Size {
        self.cell_widget.layout(ctx, &bc.loosen(), data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GridCell, env: &Env) {
        self.cell_widget.paint(ctx, data, env);
    }
}