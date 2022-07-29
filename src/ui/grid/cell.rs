use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget, WidgetExt,
};

use super::utils;
use crate::grid::{GridCell, GridCellVariant};

pub struct CellWidget {
    cell_widget: Box<dyn Widget<GridCell>>,
}

impl CellWidget {
    pub fn new() -> Self {
        Self {
            cell_widget: Box::new(utils::create_cell_svg()),
        }
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
                        utils::create_cell_svg().background(utils::get_cell_painter()), // .controller(GridCellController)
                    );
                }
            }
            _ => {}
        }

        self.cell_widget.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &GridCell, data: &GridCell, env: &Env) {
        self.cell_widget.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &GridCell,
        env: &Env,
    ) -> Size {
        self.cell_widget.layout(ctx, &bc.loosen(), data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GridCell, env: &Env) {
        self.cell_widget.paint(ctx, data, env);
    }
}
