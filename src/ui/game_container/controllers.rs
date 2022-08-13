use druid::widget::Controller;
use druid::{Env, Event, EventCtx, Widget};

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

        match event {
            Event::MouseDown(mouse_event) | Event::MouseMove(mouse_event) => {
                data.grid.is_active = if let GameState::EndState(_) = data.state {
                    false
                } else {
                    hot && (mouse_event.buttons.has_left()
                        && !mouse_event.buttons.has_middle()
                        && !mouse_event.buttons.has_right())
                        || (mouse_event.buttons.has_middle()
                            || (mouse_event.buttons.has_left() && mouse_event.buttons.has_right())
                            || (mouse_event.buttons.has_right() && mouse_event.buttons.has_left()))
                };
            }
            _ => {}
        }

        child.event(ctx, event, data, env)
    }
}
