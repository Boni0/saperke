use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, Size, UpdateCtx,
    Widget, WindowHandle,
};

use crate::{
    consts::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH},
    grid::GridSize,
};

pub struct WindowSizeObserverWidget;

impl WindowSizeObserverWidget {
    fn set_window_size(&self, window_handle: &WindowHandle, size: &GridSize) {
        window_handle.set_size(Size {
            width: (GRID_CELL_WIDTH * (size.width as f64)) + 50.0,
            height: (GRID_CELL_HEIGHT * (size.height as f64)) + 50.0,
        })
    }
}

impl Widget<GridSize> for WindowSizeObserverWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut GridSize, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &GridSize, data: &GridSize, _env: &Env) {
        self.set_window_size(ctx.window(), data);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &GridSize,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.set_window_size(ctx.window(), data);
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &GridSize,
        _env: &Env,
    ) -> Size {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &GridSize, _env: &Env) {}
}
