use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::{Controller};

use crate::grid::{GridCellState, GridCell};
use crate::delegate::{CHANGE_GRID_CELL_STATE, SET_GRID_CELLS_VISIBLE};

pub struct GridCellController;
impl<W> Controller<GridCell, W> for GridCellController where W: Widget<GridCell> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, cell: &mut GridCell, env: &Env) {
        let hot = ctx.is_hot();

        match event {
            Event::MouseDown(mouse_event) => {
                match mouse_event.button {
                    MouseButton::Left => {
                        match cell.state {
                            GridCellState::Hidden => Some(GridCellState::Active),
                            _ => None,
                        }
                    },
                    MouseButton::Right => {
                        match cell.state {
                            GridCellState::Hidden => Some(GridCellState::Tagged),
                            GridCellState::Tagged => Some(GridCellState::Questioned),
                            GridCellState::Questioned => Some(GridCellState::Hidden),
                            _ => None,
                        }
                    },
                    _ => None
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
                
            },
            _ => None
        }
        .and_then(|new_state| {
            ctx.submit_command(CHANGE_GRID_CELL_STATE.with(
                (cell.point.clone(), new_state)
            ));

            Some(())
        });

        match event {
            Event::MouseUp(mouse_event) => {
                if 
                    cell.state == GridCellState::Active &&
                    mouse_event.button == MouseButton::Left {
                    ctx.submit_command(SET_GRID_CELLS_VISIBLE.with(cell.point.clone()));
                }
            },
            _ => ()
        }

        child.event(ctx, event, cell, env)
    }
}