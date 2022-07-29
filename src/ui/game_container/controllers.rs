use druid::widget::Controller;
use druid::{Env, Event, EventCtx, MouseButton, Widget};

use crate::game::{Game, GameState};

pub struct GameContainerController;
impl<W> Controller<Game, W> for GameContainerController
where
    W: Widget<Game>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Game,
        env: &Env,
    ) {
        let hot = ctx.is_hot();

        if let GameState::EndState(_) = data.state {
        } else {
            data.grid.is_active = match event {
                Event::MouseDown(event) | Event::MouseMove(event) => {
                    hot && event.buttons.contains(MouseButton::Left)
                }
                _ => false,
            }
        }

        child.event(ctx, event, data, env)
    }
}
