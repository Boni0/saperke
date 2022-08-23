use std::time::Duration;

use druid::widget::Controller;
use druid::{Env, Event, EventCtx, Widget};

use crate::delegate::TOGGLE_PAUSE_GAME;
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
        if data.state == GameState::Paused {
            data.grid.is_active = false;

            match event {
                Event::MouseUp(_) => {
                    // Make small delay to not invoke grid opening cell logic controller
                    ctx.request_timer(Duration::from_millis(10));
                }
                Event::Timer(_) => {
                    ctx.submit_command(TOGGLE_PAUSE_GAME);
                }
                _ => (),
            }
        } else {
            match event {
                Event::MouseDown(mouse_event)
                | Event::MouseMove(mouse_event)
                | Event::MouseUp(mouse_event) => {
                    data.grid.is_active = match data.state {
                        GameState::EndState(_) => false,
                        _ => {
                            ctx.is_hot()
                                && (mouse_event.buttons.has_left()
                                    && !mouse_event.buttons.has_middle()
                                    && !mouse_event.buttons.has_right())
                                || (mouse_event.buttons.has_middle()
                                    || (mouse_event.buttons.has_left()
                                        && mouse_event.buttons.has_right())
                                    || (mouse_event.buttons.has_right()
                                        && mouse_event.buttons.has_left()))
                        }
                    }
                }
                _ => {}
            }
        }

        child.event(ctx, event, data, env)
    }
}
