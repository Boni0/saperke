mod cell;
mod controllers;
mod utils;

use crate::consts::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH};
use crate::grid::{Grid, GridCells};
use druid::widget::{Container, Flex, SizedBox};
use druid::{lens, LensExt, Widget, WidgetExt};

use controllers::GridController;

use self::cell::CellWidget;
use self::utils::{init_cell_painter, PainterSvgData};

pub struct GridWidget {
    svg_data: PainterSvgData,
    inner: Box<dyn Widget<Grid>>,
}

impl GridWidget {
    pub fn new() -> Self {
        Self {
            svg_data: init_cell_painter(),
            inner: SizedBox::empty().boxed(),
        }
    }

    fn create_inner(&mut self, data: &Grid) {
        let mut main_flex = Flex::column();

        for row_idx in 0..data.size.height {
            let mut row_flex = Flex::row();

            for cell_idx in 0..data.size.width {
                row_flex.add_child(
                    CellWidget::new()
                        .background(CellWidget::painter(self.svg_data.clone()))
                        .lens(lens::Identity.index(cell_idx)),
                );
            }

            main_flex.add_child(row_flex.lens(lens::Identity.index(row_idx)));
        }

        self.inner =
            Container::new(main_flex.lens(lens!(Grid, cells).then(lens!(GridCells, matrix))))
                .controller(GridController::new())
                .boxed()
    }
}

impl Widget<Grid> for GridWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Grid,
        env: &druid::Env,
    ) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Grid,
        env: &druid::Env,
    ) {
        match event {
            druid::LifeCycle::WidgetAdded => {
                self.create_inner(data);
                ctx.children_changed();
            }
            _ => (),
        }

        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Grid,
        data: &Grid,
        env: &druid::Env,
    ) {
        if old_data.size.height != data.size.height || old_data.size.width != data.size.width {
            self.create_inner(data);
            ctx.children_changed();
        } else {
            self.inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Grid,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env);

        druid::Size {
            width: GRID_CELL_WIDTH * (data.size.width as f64),
            height: GRID_CELL_HEIGHT * (data.size.height as f64),
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Grid, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}
