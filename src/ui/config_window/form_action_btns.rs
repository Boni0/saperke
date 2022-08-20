use crate::{
    consts::{
        CUSTOM_GAME_CANCEL_BTN_LABEL, CUSTOM_GAME_CONFIRM_BTN_LABEL, GAME_MAX_HEIGHT,
        GAME_MAX_WIDTH, GAME_MIN_HEIGHT, GAME_MIN_WIDTH,
    },
    delegate::{NEW_GAME_SIMPLE_CUSTOM_BOX, NEW_GAME_SIMPLE_UNUSUAL},
    grid::{GridSize, GridStartShape},
    ui::border_box::{BorderBox, BorderColorPattern},
};

use druid::{
    commands::CLOSE_WINDOW,
    widget::{Flex, Label, Painter, SizedBox},
    Color, Widget, WidgetExt,
};

use super::ConfigWindow;

pub struct ConfigWindowActionBtns;
impl ConfigWindowActionBtns {
    pub fn new() -> impl Widget<ConfigWindow> {
        let make_btn_painter = || {
            Painter::<ConfigWindow>::new(|ctx, data, env| {
                let pattern = if ctx.is_active() {
                    BorderColorPattern::DarkerFirst
                } else {
                    BorderColorPattern::LigherFirst
                };

                BorderBox::new_with_custom_size(SizedBox::empty(), pattern, 2.0)
                    .paint(ctx, data, env);
            })
        };

        let ok_btn = SizedBox::new(
            Label::new(CUSTOM_GAME_CONFIRM_BTN_LABEL)
                .with_text_color(Color::BLACK)
                .center(),
        )
        .background(make_btn_painter())
        .fix_size(80.0, 35.0)
        .on_click(|ctx, config: &mut ConfigWindow, _| {
            let bombs_amount = config.custom_bombs_amount.parse::<usize>().unwrap_or(0);

            if let GridStartShape::Unusual(variant) = &config.custom_start_shape {
                ctx.submit_command(NEW_GAME_SIMPLE_UNUSUAL.with((variant.clone(), bombs_amount)));
                ctx.submit_command(CLOSE_WINDOW);
            } else {
                let width = config
                    .custom_width
                    .parse::<usize>()
                    .unwrap_or(0)
                    .min(GAME_MAX_WIDTH);

                let height = config
                    .custom_height
                    .parse::<usize>()
                    .unwrap_or(0)
                    .min(GAME_MAX_HEIGHT);

                if width >= GAME_MIN_WIDTH && height >= GAME_MIN_HEIGHT && bombs_amount > 0 {
                    ctx.submit_command(
                        NEW_GAME_SIMPLE_CUSTOM_BOX.with((GridSize { width, height }, bombs_amount)),
                    );
                    ctx.submit_command(CLOSE_WINDOW);
                }
            }
        });

        let cancel_btn = SizedBox::new(
            Label::new(CUSTOM_GAME_CANCEL_BTN_LABEL)
                .with_text_color(Color::BLACK)
                .center(),
        )
        .background(make_btn_painter())
        .fix_size(80.0, 35.0)
        .on_click(|ctx, _, _| ctx.submit_command(CLOSE_WINDOW));

        Flex::column()
            .with_child(ok_btn)
            .with_spacer(15.0)
            .with_child(cancel_btn)
    }
}
