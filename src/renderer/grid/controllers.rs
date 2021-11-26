use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::{Controller};

use crate::grid::{GridCellState, GridCell};

pub struct GridCellController;
impl<W> Controller<GridCell, W> for GridCellController where W: Widget<GridCell> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, cell: &mut GridCell, env: &Env) {
        let hot = ctx.is_hot();

        match event {
            Event::MouseMove(mouse_event) => {
                if cell.state != GridCellState::Visible {
                    cell.state = if hot && mouse_event.buttons.contains(MouseButton::Left) {
                        GridCellState::Active
                    } else {
                        GridCellState::Hidden
                    }
                }
            },
            _ => ()
        };


        child.event(ctx, event, cell, env)
    }
}