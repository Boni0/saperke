use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::{Controller};

use crate::grid::{GridCellState, GridCell, GridCellVariant};
use crate::delegate::{
    GRID_SET_CELLS_VISIBLE,
    GRID_SET_CELL_STATE
};

pub struct GridCellController;
impl<W> Controller<GridCell, W> for GridCellController where W: Widget<GridCell> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, cell: &mut GridCell, env: &Env) {
        let hot = ctx.is_hot();

        if let GridCellVariant::Exist(cell_data) = &cell.variant {
            match event {
                Event::MouseDown(mouse_event) => {
                    match mouse_event.button {
                        MouseButton::Left => {
                            match cell_data.state {
                                GridCellState::Idle => Some(GridCellState::Active),
                                _ => None,
                            }
                        },
                        MouseButton::Right => {
                            match cell_data.state {
                                GridCellState::Idle => Some(GridCellState::Tagged),
                                GridCellState::Tagged => Some(GridCellState::Questioned),
                                GridCellState::Questioned => Some(GridCellState::Idle),
                                _ => None,
                            }
                        },
                        _ => None
                    }
                },
                
                Event::MouseMove(mouse_event) => {
                    match cell_data.state {
                        GridCellState::Idle => {
                            if hot && mouse_event.buttons.contains(MouseButton::Left) {
                                Some(GridCellState::Active)
                            } else {
                                None
                            }
                        },
                        GridCellState::Active => {
                            if !hot {
                                Some(GridCellState::Idle)
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
                ctx.submit_command(GRID_SET_CELL_STATE.with(
                    (cell.point.clone(), new_state)
                ));

                Some(())
            });

            match event {
                Event::MouseUp(mouse_event) => {
                    if 
                        cell_data.state == GridCellState::Active &&
                        mouse_event.button == MouseButton::Left {
                        ctx.submit_command(GRID_SET_CELLS_VISIBLE.with(cell.point.clone()));
                    }
                },
                _ => ()
            }
        }

        child.event(ctx, event, cell, env)
    }
}