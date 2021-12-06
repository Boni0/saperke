use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::Controller;

use crate::grid::{GridCellState, GridCell, GridCellVariant, GridCellFlaggedState};
use crate::delegate::{
    HANDLE_CELL_FLAGGING,
    HANDLE_CELL_OPEN,
    HANDLE_CELL_TOGGLE_HOVER,
};

pub struct GridCellController;
impl<W> Controller<GridCell, W> for GridCellController where W: Widget<GridCell> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, cell: &mut GridCell, env: &Env) {
        let hot = ctx.is_hot();

        if let GridCellVariant::Exist(cell_data) = &mut cell.variant {
            if !cell_data.is_visible {

                // Idle/Hover state toggle
                if let Some(new_state) = match event {
                    Event::MouseDown(mouse_event) => {
                        match mouse_event.button {
                            MouseButton::Left => {
                                match cell_data.state {
                                    GridCellState::Idle => Some(GridCellState::Active),
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
                } {
                    ctx.submit_command(HANDLE_CELL_TOGGLE_HOVER.with(
                        (cell.point.clone(), new_state)
                    ));
                }

                // Flagged State
                if let Event::MouseDown(mouse_event) = event {
                    if let MouseButton::Right = mouse_event.button {
                        if let Some(new_flagged_state_option) = match cell_data.state {
                            GridCellState::Idle => Some(Some(GridCellFlaggedState::Tagged)),
                            GridCellState::Flagged(GridCellFlaggedState::Tagged) => Some(Some(GridCellFlaggedState::Questioned)),
                            GridCellState::Flagged(GridCellFlaggedState::Questioned) => Some(None),
                            _ => None,
                        } {
                            ctx.submit_command(HANDLE_CELL_FLAGGING.with(
                                (cell.point.clone(), new_flagged_state_option)
                            ));
                        }
                    }
                }

                // Open cell
                if let Event::MouseUp(mouse_event) = event {
                    if 
                        cell_data.state == GridCellState::Active &&
                        mouse_event.button == MouseButton::Left {
                            cell_data.state = GridCellState::Idle;
                            ctx.submit_command(HANDLE_CELL_OPEN.with( cell.point.clone() ));
                    }
                }

            }
           
        }

        child.event(ctx, event, cell, env)
    }
}