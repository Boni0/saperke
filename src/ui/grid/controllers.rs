use druid::widget::Controller;
use druid::{Env, Event, EventCtx, MouseButton, Point, Widget};

use crate::consts::{GRID_CELL_HEIGHT, GRID_CELL_WIDTH};

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
        if let Some((cell_point, mouse_event)) = match event {
            Event::MouseDown(event) | Event::MouseUp(event) | Event::MouseMove(event) => {
                let Point { x, y } = event.pos;
                let x = x / GRID_CELL_WIDTH;
                let y = y / GRID_CELL_HEIGHT;

                if x >= 0.0
                    && x <= (data.size.width as f64)
                    && y > 0.0
                    && y <= (data.size.height as f64)
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

            if !old_active_points.is_empty() {
                ctx.submit_command(CELL_IDLE_BY_MULTIPLE_POINTS.with(old_active_points));
            }

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
            if !self.last_active_cell_points.is_empty() {
                ctx.submit_command(
                    CELL_IDLE_BY_MULTIPLE_POINTS.with(self.last_active_cell_points.clone()),
                );
                self.last_active_cell_points = Vec::new();
            }
        }

        child.event(ctx, event, data, env)
    }
}
