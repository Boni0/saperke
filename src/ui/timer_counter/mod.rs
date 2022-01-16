use std::convert::TryInto;
use std::time::Duration;

use druid::{Widget, WidgetExt, TimerToken, lens, LensExt, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx};
use druid::widget::SizedBox;

use crate::app::AppState;
use crate::game::GameState;
use crate::ui::ThreeColumnCounter;

pub struct TimerCounter {
    pub timer_id: TimerToken,
    pub inner: Box<dyn Widget<AppState>>
}

static TIMER_INTERVAL: Duration = Duration::from_millis(100);

impl TimerCounter {
    pub fn new() -> Self {
        let inner = SizedBox::new(ThreeColumnCounter::new())
        .lens(lens::Identity.map(
        |state: &AppState| state.game.time.as_secs().try_into().unwrap(),
        |_, _| {}
        ));

        Self {
            timer_id: TimerToken::INVALID,
            inner: Box::new(inner)
        }
    }
}

impl Widget<AppState> for TimerCounter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Timer(id) => {
                if *id == self.timer_id {
                    data.game.time += TIMER_INTERVAL;
                    ctx.request_layout();
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                    println!("Time: {:?}", data.game.time);
                }
            }
            _ => (),
        }

        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        self.inner.update(ctx, old_data, data, env);

        match data.game.state {
            GameState::Running => {
                if old_data.game.state != GameState::Running {
                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            },
            GameState::NotStarted | GameState::Paused | GameState::EndState(_) => {
                self.timer_id = TimerToken::INVALID
            }
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &AppState, env: &Env) -> Size {
        self.inner.layout(ctx, &bc.loosen(), data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}