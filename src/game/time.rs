use druid::{Data, Lens};
use std::time::Duration;

#[derive(Clone, Data, Lens)]
pub struct GameTime {
    pub display_time_in_sec: usize,
    #[data(same_fn = "PartialEq::eq")]
    pub final_time_duration: Duration,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            display_time_in_sec: 0,
            final_time_duration: Duration::from_millis(0),
        }
    }

    pub fn reset(&mut self) {
        let new_instance = GameTime::new();
        self.display_time_in_sec = new_instance.display_time_in_sec;
        self.final_time_duration = new_instance.final_time_duration;
    }
}
