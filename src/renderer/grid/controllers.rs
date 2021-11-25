use druid::{Env, Event, EventCtx, MouseButton, Widget};
use druid::widget::{Controller, Svg, Painter};

use crate::AppStruct;
use crate::grid::GridCellState;

pub struct CellClickController {
    pub y_cord: usize,
    pub x_cord: usize,
}

impl CellClickController {
    pub fn init(y_cord: usize, x_cord: usize) -> Self {
        Self {
            y_cord,
            x_cord
        }
    }
}

impl<W> Controller<AppStruct, W> for CellClickController where W: Widget<AppStruct> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, app: &mut AppStruct, env: &Env) {
        let hot = ctx.is_hot();

        match event {
            // Event::WindowConnected => todo!(),
            // Event::WindowSize(_) => todo!(),
            Event::MouseDown(event) => {
                let cell = app.game.grid.get_cell(self.y_cord, self.x_cord).unwrap();
                println!("clicked at y:{} x:{}", self.y_cord, self.x_cord);
            },
            Event::MouseUp(_) => {
                println!("un clicked at y:{} x:{}", self.y_cord, self.x_cord);
            },
            Event::MouseMove(mouse_event) => {
                // if hot {
                //     // let cell = app.game.grid.get_cell(self.y_cord, self.x_cord).unwrap();
                // }
                println!("is hot at y:{} x:{} - {}", self.y_cord, self.x_cord, hot);
                
                if !hot {
                    let cell = app.game.grid.get_cell(self.y_cord, self.x_cord).unwrap();
                    if cell.state == GridCellState::Clicked {
                        cell.state = GridCellState::Hidden;
                    }
                    // println!("not hot at y:{} x:{}", self.y_cord, self.x_cord);
                } else {
                    let grid = &mut app.game.grid;
                    let cell = &grid.cells.get_mut(1);
                }

                println!("is hot at y:{} x:{} - {}", self.y_cord, self.x_cord, hot);


                // let cell = app.game.grid.get_cell(self.y_cord, self.x_cord).unwrap();

                // if mouse_event.buttons.contains(MouseButton::Left) && hot && cell.state == GridCellState::Hidden {
                //     cell.state = GridCellState::Clicked;
                // }

                // if !hot && cell.state == GridCellState::Clicked {
                //     cell.state = GridCellState::Hidden;
                // }


                // println!("move at y:{} x:{}, hot? {}", self.y_cord, self.x_cord, ctx.is_hot());
                // println!("ctx is_active:{}, is_hot:{}, is_focused:{}, is_handled:{}, left clicked: {}", ctx.is_active(), ctx.is_hot(), ctx.is_focused(), ctx.is_handled(), mouse_event.buttons.contains(MouseButton::Left));

                // let cell = app.game.grid.get_cell(self.y_cord, self.x_cord).unwrap();

                // if mouse_event.buttons.contains(MouseButton::Left) &&
                //     ctx.is_hot() &&
                //     cell.state == GridCellState::Hidden 
                // {
                //     cell.state = GridCellState::Clicked;
                // }

                // if !ctx.is_hot() &&
                //     cell.state == GridCellState::Clicked 
                // {
                //     cell.state = GridCellState::Hidden;
                // }

            },
            // Event::Wheel(_) => todo!(),
            // Event::KeyDown(_) => todo!(),
            // Event::KeyUp(_) => todo!(),
            // Event::Paste(_) => todo!(),
            // Event::Zoom(_) => todo!(),
            // Event::Timer(_) => todo!(),
            // Event::AnimFrame(_) => todo!(),
            // Event::Command(_) => todo!(),
            // Event::Notification(_) => todo!(),
            // Event::Internal(_) => todo!(),
            _ => ()
        };

        child.event(ctx, event, app, env)
    }
}