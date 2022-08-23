use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
    time::Duration,
};

use druid::{
    BoxConstraints, Env, Event, EventCtx, ExtEventSink, LayoutCtx, LifeCycle, LifeCycleCtx,
    Selector, Size, Target, UpdateCtx, Widget, WidgetId,
};

use crate::{
    consts::TIMER_INTERVAL,
    game::{Game, GameState},
};

pub struct TimerObserver {
    time_duration: Duration,
    channel_sender: Option<Sender<GameState>>,
}

impl TimerObserver {
    pub const INIT_TIME_DURATION: Duration = Duration::from_millis(0);

    pub fn new() -> Self {
        Self {
            time_duration: TimerObserver::INIT_TIME_DURATION,
            channel_sender: None,
        }
    }

    pub fn send_game_state(&mut self, new_game_state: GameState) {
        if let Some(sender) = &self.channel_sender {
            sender.send(new_game_state).unwrap();
        }
    }

    pub fn is_time_duration_full_sec(&self) -> bool {
        self.time_duration.as_millis() % 1000 == 0
    }
}

const TIMER_OBSERVER_SELECTOR: Selector<()> = Selector::new("TIMER_OBSERVER_SELECTOR");

fn create_timer_observer_thread(
    event_sink: ExtEventSink,
    widget_id: WidgetId,
    receiver: Receiver<GameState>,
) {
    loop {
        match receiver.try_recv() {
            Ok(GameState::Running) | Err(TryRecvError::Empty) => {
                thread::sleep(TIMER_INTERVAL);
                event_sink
                    .submit_command(TIMER_OBSERVER_SELECTOR, (), Target::Widget(widget_id))
                    .unwrap();
            }
            _ => {
                break;
            }
        }
    }
}

impl Widget<Game> for TimerObserver {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, game: &mut Game, _env: &Env) {
        match event {
            Event::Command(command) => {
                if command.is(TIMER_OBSERVER_SELECTOR) {
                    if game.state == GameState::Running {
                        self.time_duration += TIMER_INTERVAL;

                        if self.is_time_duration_full_sec() {
                            game.time.display_time_in_sec = self.time_duration.as_secs() as usize;
                        }
                    } else {
                        self.send_game_state(game.state.clone());

                        if let GameState::EndState(_) = game.state {
                            if self.channel_sender.is_some() {
                                game.time.final_time_duration = Duration::from(self.time_duration);
                            }
                        }

                        self.channel_sender = None;
                        if game.state != GameState::Paused {
                            self.time_duration = TimerObserver::INIT_TIME_DURATION;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_game_data: &Game, game: &Game, _env: &Env) {
        if old_game_data.state == GameState::Paused && game.state == GameState::NotStarted {
            self.time_duration = TimerObserver::INIT_TIME_DURATION;
        }

        if game.state == GameState::Running && self.channel_sender.is_none() {
            let (sender, receiver) = mpsc::channel::<GameState>();

            sender.send(GameState::Running).unwrap();
            self.channel_sender = Some(sender);

            let event_sink = ctx.get_external_handle();
            let id = ctx.widget_id();
            thread::spawn(move || create_timer_observer_thread(event_sink, id, receiver));
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Game, _env: &Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &Game,
        _env: &Env,
    ) -> Size {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &Game, _env: &Env) {}
}
