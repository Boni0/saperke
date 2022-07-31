mod utils;

use std::convert::TryInto;

use druid::widget::{Flex, SizedBox};
use druid::{lens, LensExt, Widget, WidgetExt};

use crate::app::AppState;
use crate::consts::{FLEX_COMMON_SPACING_SIZE, TIMER_COLUMN_HEIGHT};
use crate::game::Game;
use crate::ui::ThreeColumnCounter;

use self::utils::{get_btn_icon_face_painter, get_btn_painter};

use super::border_box::{BorderBox, BorderColorPattern};

pub struct InfoPanel;

impl InfoPanel {
    pub fn new() -> impl Widget<AppState> {
        let mut flex = Flex::row();

        flex.add_child(ThreeColumnCounter::new().lens(lens::Identity.map(
            |state: &AppState| {
                let bombs_count: i64 = state.game.grid.bombs.count.try_into().unwrap();
                let tagged_count: i64 = state
                    .game
                    .grid
                    .cells
                    .tagged_points
                    .len()
                    .try_into()
                    .unwrap();
                bombs_count - tagged_count
            },
            |_, _| {},
        )));

        let mut face_btn = Flex::column();
        face_btn.add_child(
            SizedBox::empty()
                .fix_size(25.5, 25.5)
                .background(get_btn_icon_face_painter()),
        );

        flex.add_child(
            face_btn
                .center()
                .fix_size(TIMER_COLUMN_HEIGHT, TIMER_COLUMN_HEIGHT)
                .background(get_btn_painter())
                .on_click(|_, game: &mut Game, _| game.restart())
                .lens(lens!(AppState, game)),
        );

        flex.add_child(ThreeColumnCounter::new().lens(lens::Identity.map(
            |state: &AppState| state.game.time.as_secs().try_into().unwrap(),
            |_, _| {},
        )));

        BorderBox::new(
            flex.main_axis_alignment(druid::widget::MainAxisAlignment::SpaceBetween)
                .padding(FLEX_COMMON_SPACING_SIZE / 2.0),
            BorderColorPattern::DarkerFirst,
        )
    }
}
