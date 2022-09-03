use druid::{
    commands::CLOSE_WINDOW,
    widget::{Flex, Label, LineBreaking, MainAxisAlignment, Painter, SizedBox},
    Color, FontDescriptor, FontWeight, Widget, WidgetExt,
};

use crate::{
    app::AppState,
    consts::{
        ABOUT_BASEMENT_TITLE, ABOUT_BASEMENT_URL, ABOUT_DESC, ABOUT_MUA, ABOUT_MUA_INTRO, TITLE,
    },
};

use webbrowser;

use super::border_box::{BorderBox, BorderColorPattern};

pub struct AboutWindow;
impl AboutWindow {
    pub fn new() -> impl Widget<AppState> {
        let mut flex = Flex::column();

        let desc = Label::new(format!("{}:{}", TITLE, ABOUT_DESC))
            .with_text_color(Color::BLACK)
            .with_line_break_mode(LineBreaking::WordWrap);

        let mua = Flex::column()
            .with_child(
                Label::new(ABOUT_MUA_INTRO)
                    .with_text_color(Color::BLACK)
                    .with_line_break_mode(LineBreaking::WordWrap),
            )
            .with_spacer(5.0)
            .with_child(
                Label::new(ABOUT_MUA)
                    .with_text_color(Color::BLACK)
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .with_font(
                        FontDescriptor::default()
                            .with_weight(FontWeight::BOLD)
                            .with_size(16.0),
                    ),
            );

        let click_me =
            SizedBox::new(Label::new(ABOUT_BASEMENT_TITLE).with_text_color(Color::BLACK))
                .padding(10.0)
                .background(Painter::new(|ctx, data, env| {
                    let pattern = if ctx.is_active() {
                        BorderColorPattern::DarkerFirst
                    } else {
                        BorderColorPattern::LigherFirst
                    };

                    BorderBox::new_with_custom_size(SizedBox::empty(), pattern, 2.0)
                        .paint(ctx, data, env);
                }))
                .on_click(|ctx, _, _| {
                    if webbrowser::open(ABOUT_BASEMENT_URL).is_ok() {
                        ctx.submit_command(CLOSE_WINDOW);
                    }
                })
                .center();

        flex.add_child(desc);
        flex.add_spacer(10.0);
        flex.add_child(mua);
        flex.add_spacer(10.0);
        flex.add_child(click_me);

        flex.main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .padding(20.0)
    }
}
