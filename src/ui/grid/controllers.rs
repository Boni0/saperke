use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::{Controller};

use crate::grid::{GridCellState, GridCell};
use crate::delegate::{CHANGE_GRID_CELL_STATE};

pub struct GridCellController;
impl<W> Controller<GridCell, W> for GridCellController where W: Widget<GridCell> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, cell: &mut GridCell, env: &Env) {
        let hot = ctx.is_hot();

        match event {
            Event::MouseDown(_mouse_event) => {
                if hot && cell.state == GridCellState::Hidden {
                    ctx.submit_command(CHANGE_GRID_CELL_STATE.with(
                        (cell.point.clone(), GridCellState::Active)
                    ));
                }
            },
            Event::MouseUp(_mouse_event) => {
                if hot && cell.state == GridCellState::Active {
                    ctx.submit_command(CHANGE_GRID_CELL_STATE.with(
                        (cell.point.clone(), GridCellState::Hidden)
                    ));
                }
            },
            Event::MouseMove(mouse_event) => {
                match cell.state {
                    GridCellState::Hidden => {
                        if hot && mouse_event.buttons.contains(MouseButton::Left) {
                            Some(GridCellState::Active)
                        } else {
                            None
                        }
                    },
                    GridCellState::Active => {
                        if !hot {
                            Some(GridCellState::Hidden)
                        } else {
                            None
                        }
                    },
                    _ => None
                }
                .and_then(|new_state| {
                    ctx.submit_command(CHANGE_GRID_CELL_STATE.with(
                        (cell.point.clone(), new_state)
                    ));

                    Some(())
                });
            },
            _ => ()
        };


        child.event(ctx, event, cell, env)
    }
}