mod controllers;
mod cell;
mod utils;

use druid::{LensExt, Widget, WidgetExt, lens, LifeCycleCtx, LifeCycle, Env, UpdateCtx};
use druid::widget::List;

use crate::app::AppState;
use crate::game::Game;
use crate::grid::{GridCells, Grid, GridSize, GridCellMatrix};

use controllers::GridCellController;
use cell::CellWidget;

pub struct GridWidget {
    list: Box<dyn Widget<Grid>>,
    last_size: GridSize
}

impl GridWidget {
    pub fn new() -> Self {
        Self { 
            list: Box::new(
                List::new(|| {
                    List::new(|| {
                        CellWidget::new()
                    })
                    .horizontal()
                })
                .lens(
                    lens!(Grid, cells)
                        .then(lens!(GridCells, matrix))
                )
            ), 
            last_size: GridSize { width: 0, height: 0 }
        }
    }
}

impl Widget<Grid> for GridWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut Grid, env: &Env) {
        self.list.event(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Grid, data: &Grid, env: &Env) {
        self.list.update(ctx, old_data, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Grid, env: &Env) {
        self.list.lifecycle(ctx, event, data, env);
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &Grid, env: &Env) -> druid::Size {
        self.list.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Grid, env: &Env) {
        self.list.paint(ctx, data, env);
    }
}