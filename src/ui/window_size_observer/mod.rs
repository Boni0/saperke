use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, Size, UpdateCtx,
    Widget, WindowHandle,
};

use crate::{
    consts::{
        BORDER_SIZE, FLEX_COMMON_SPACING_SIZE, GRID_CELL_HEIGHT, GRID_CELL_WIDTH,
        TIMER_COLUMN_HEIGHT,
    },
    grid::GridSize,
};

const HORIZONTAL_SPACING: f64 = FLEX_COMMON_SPACING_SIZE * 2.0;
const HORIZONTAL_BORDERS: f64 = BORDER_SIZE * 4.0;

const VERTICAL_SPACING: f64 = FLEX_COMMON_SPACING_SIZE * 3.0;
const VERTICAL_BORDERS: f64 = BORDER_SIZE * 6.0;
const VERTICAL_EXTRA_SIZES: f64 = TIMER_COLUMN_HEIGHT + FLEX_COMMON_SPACING_SIZE;

pub struct WindowSizeObserverWidget;
impl WindowSizeObserverWidget {
    fn set_window_size(&self, window_handle: &WindowHandle, size: &GridSize) {
        window_handle.set_size(Size {
            width: (GRID_CELL_WIDTH * (size.width as f64))
                + HORIZONTAL_SPACING
                + HORIZONTAL_BORDERS,
            height: (GRID_CELL_HEIGHT * (size.height as f64))
                + VERTICAL_SPACING
                + VERTICAL_BORDERS
                + VERTICAL_EXTRA_SIZES,
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
