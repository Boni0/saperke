use druid::widget::Controller;
use druid::{Env, Event, EventCtx, MouseButton, Point, Widget};

use crate::consts::GRID_CELL_WIDTH;

use crate::delegate::{
    CELL_ACTIVE_BY_MULTIPLE_POINTS, CELL_IDLE_BY_MULTIPLE_POINTS, CELL_OPEN_BY_POINT,
    CELL_TOGGLE_FLAG_BY_POINT,
};

use crate::grid::{Grid, GridCellPoint};

pub struct GridController {
    pub last_button_clicked: MouseButton,
    pub last_active_cell_points: Vec<GridCellPoint>,
}

impl GridController {
    pub fn new() -> Self {
        Self {
            last_button_clicked: MouseButton::None,
            last_active_cell_points: Vec::new(),
        }
    }
}

impl<W> Controller<Grid, W> for GridController
where
    W: Widget<Grid>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Grid,
        env: &Env,
    ) {
        // let hot = ctx.is_hot();

        if let Some((cell_point, mouse_event)) = match event {
            Event::MouseDown(event) | Event::MouseUp(event) | Event::MouseMove(event) => {
                let Point { x, y } = event.pos;
                let x = x / GRID_CELL_WIDTH;
                let y = y / GRID_CELL_WIDTH;

                if x >= 0.0
                    && x <= (data.size.width as f64)
                    && y > 0.0
                    && x <= (data.size.height as f64)
                {
                    let grid_cell_point = GridCellPoint {
                        x: x as usize,
                        y: y as usize,
                    };

                    data.cells
                        .get_existing_cell(&grid_cell_point)
                        .and_then(|_| Some((grid_cell_point, event)))
                } else {
                    None
                }
            }
            _ => return,
        } {
            let mut new_active_points = Vec::new();

            if mouse_event.buttons.has_left()
                && !mouse_event.buttons.has_middle()
                && !mouse_event.buttons.has_right()
            {
                new_active_points.push(cell_point.clone());
            } else if mouse_event.buttons.has_middle()
                || (mouse_event.buttons.has_left() && mouse_event.buttons.has_right())
                || (mouse_event.buttons.has_right() && mouse_event.buttons.has_left())
            {
                let get_dimension_cord = |point: usize, max_point: usize| {
                    let start = point - (if point == 0 { 0 } else { 1 });

                    let end = start
                        + (if point == 0 || point == max_point {
                            2
                        } else {
                            3
                        });

                    (start, end)
                };

                let (start_x, end_x) = get_dimension_cord(cell_point.x, data.size.width);
                let (start_y, end_y) = get_dimension_cord(cell_point.y, data.size.height);

                for current_y in start_y..end_y {
                    for current_x in start_x..end_x {
                        new_active_points.push(GridCellPoint {
                            x: current_x,
                            y: current_y,
                        });
                    }
                }
            }

            let old_active_points: Vec<GridCellPoint> = self
                .last_active_cell_points
                .clone()
                .into_iter()
                .filter(|active_cell| !new_active_points.contains(active_cell))
                .collect();

            ctx.submit_command(CELL_IDLE_BY_MULTIPLE_POINTS.with(old_active_points));

            if !new_active_points.is_empty() {
                ctx.submit_command(CELL_ACTIVE_BY_MULTIPLE_POINTS.with(new_active_points.clone()));
                self.last_active_cell_points = new_active_points;
                return;
            }

            if let Event::MouseDown(_) = event {
                if mouse_event.button.is_right() {
                    ctx.submit_command(CELL_TOGGLE_FLAG_BY_POINT.with(cell_point));
                    return;
                }
            }

            if let Event::MouseUp(_) = event {
                if mouse_event.button.is_left() && mouse_event.buttons.is_empty() {
                    ctx.submit_command(CELL_OPEN_BY_POINT.with(cell_point));
                    return;
                }
            }
        } else {
            ctx.submit_command(
                CELL_IDLE_BY_MULTIPLE_POINTS.with(self.last_active_cell_points.clone()),
            );
            self.last_active_cell_points = Vec::new();
        }

        child.event(ctx, event, data, env)
    }
}

// pub struct GridCellController;
// impl<W> Controller<GridCell, W> for GridCellController
// where
//     W: Widget<GridCell>,
// {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut EventCtx,
//         event: &Event,
//         cell: &mut GridCell,
//         env: &Env,
//     ) {
//         let hot = ctx.is_hot();

//         let grid_is_active = false;

//         // ctx.submit_command(HANDLE_ACTIVE_GRID.with(grid_is_active));

//         if let GridCellVariant::Exist(cell_data) = &mut cell.variant {
//             // match event {
//             //     Event::MouseDown(event) | Event::MouseMove(event) => {
//             //         hot && event.buttons.contains(MouseButton::Left)
//             //     }
//             //     _ => false,
//             // };

//             if !cell_data.is_visible {
//                 // Idle/Hover state toggle
//                 if let Some(new_state) = match event {
//                     Event::MouseDown(mouse_event) => match mouse_event.button {
//                         MouseButton::Left => match cell_data.state {
//                             GridCellState::Idle => Some(GridCellState::Active),
//                             _ => None,
//                         },
//                         _ => None,
//                     },
//                     Event::MouseMove(mouse_event) => match cell_data.state {
//                         GridCellState::Idle => {
//                             if hot && mouse_event.buttons.contains(MouseButton::Left) {
//                                 Some(GridCellState::Active)
//                             } else {
//                                 None
//                             }
//                         }
//                         GridCellState::Active => {
//                             if !hot {
//                                 Some(GridCellState::Idle)
//                             } else {
//                                 None
//                             }
//                         }
//                         _ => None,
//                     },
//                     _ => None,
//                 } {
//                     ctx.submit_command(
//                         HANDLE_CELL_TOGGLE_HOVER.with((cell.point.clone(), new_state)),
//                     );
//                 }

//                 // Flagged State
//                 if let Event::MouseDown(mouse_event) = event {
//                     if let MouseButton::Right = mouse_event.button {
//                         if let Some(new_flagged_state_option) = match cell_data.state {
//                             GridCellState::Idle => Some(Some(GridCellFlaggedState::Tagged)),
//                             GridCellState::Flagged(GridCellFlaggedState::Tagged) => {
//                                 Some(Some(GridCellFlaggedState::Questioned))
//                             }
//                             GridCellState::Flagged(GridCellFlaggedState::Questioned) => Some(None),
//                             _ => None,
//                         } {
//                             ctx.submit_command(
//                                 HANDLE_CELL_FLAGGING
//                                     .with((cell.point.clone(), new_flagged_state_option)),
//                             );
//                         }
//                     }
//                 }

//                 // Open cell
//                 if let Event::MouseUp(mouse_event) = event {
//                     if cell_data.state == GridCellState::Active
//                         && mouse_event.button == MouseButton::Left
//                     {
//                         cell_data.state = GridCellState::Idle;
//                         ctx.submit_command(HANDLE_CELL_OPEN.with(cell.point.clone()));
//                     }
//                 }

//                 // println!("grid is active: {}", grid_is_active);

//                 // ctx.submit_command(HANDLE_ACTIVE_GRID.with(grid_is_active));
//             }
//         }

//         child.event(ctx, event, cell, env)
//     }
// }
