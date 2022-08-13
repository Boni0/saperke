use druid::{
    widget::{Either, Label, SizedBox},
    Color, Widget, WidgetExt,
};

use crate::game::Game;

pub struct FinalTimeStatus;

impl FinalTimeStatus {
    pub fn new() -> impl Widget<Game> {
        let mut label = Label::<Game>::dynamic(|data, _| {
            let sec = data.time.final_time_duration.as_secs();
            let millis = data.time.final_time_duration.subsec_millis();
            format!("Final Time {}:{}:{}", sec / 60, sec % 60, millis)
        });
        label.set_text_color(Color::BLACK);
        label.set_text_size(14.0);

        Either::new(
            |game: &Game, _| !(game.time.final_time_duration.as_millis() == 0),
            label,
            SizedBox::empty().padding(5.0),
        )
    }
}
