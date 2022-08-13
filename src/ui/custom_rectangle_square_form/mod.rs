use crate::{
    consts::{
        CUSTOM_GAME_BOMBS_FROM_LABEL, CUSTOM_GAME_CANCEL_BTN_LABEL, CUSTOM_GAME_CONFIRM_BTN_LABEL,
        CUSTOM_GAME_HEIGHT_FROM_LABEL, CUSTOM_GAME_WIDTH_FROM_LABEL, GAME_MAX_HEIGHT,
        GAME_MAX_WIDTH, GAME_MIN_HEIGHT, GAME_MIN_WIDTH,
    },
    custom_rectangle_square::CustomRectangleOrSquarFormState,
    delegate::NEW_GAME_CUSTOM_RECTANGLE_OR_SQUARE,
};

use druid::{
    commands::CLOSE_WINDOW,
    text::{EditableText, TextStorage},
    widget::{Flex, Label, Painter, SizedBox, TextBox},
    Color, Data, Lens, Widget, WidgetExt,
};

use super::border_box::{BorderBox, BorderColorPattern};

pub struct CustomRectangleOrSquareSubWindow;
impl CustomRectangleOrSquareSubWindow {
    fn form_field<
        D: Data + TextStorage + EditableText,
        L: Lens<CustomRectangleOrSquarFormState, D> + 'static,
    >(
        label_txt: &str,
        lens: L,
    ) -> impl Widget<CustomRectangleOrSquarFormState> {
        Flex::<CustomRectangleOrSquarFormState>::row()
            .with_child(
                Label::new(format!("{}: ", label_txt))
                    .with_text_color(Color::BLACK)
                    .fix_width(75.0),
            )
            .with_child(TextBox::new().fix_size(50.0, 25.0).lens(lens))
    }

    pub fn new() -> impl Widget<CustomRectangleOrSquarFormState> {
        let fields = Flex::column()
            .with_child(CustomRectangleOrSquareSubWindow::form_field(
                CUSTOM_GAME_WIDTH_FROM_LABEL,
                CustomRectangleOrSquarFormState::width_string,
            ))
            .with_spacer(10.0)
            .with_child(CustomRectangleOrSquareSubWindow::form_field(
                CUSTOM_GAME_HEIGHT_FROM_LABEL,
                CustomRectangleOrSquarFormState::height_string,
            ))
            .with_spacer(10.0)
            .with_child(CustomRectangleOrSquareSubWindow::form_field(
                CUSTOM_GAME_BOMBS_FROM_LABEL,
                CustomRectangleOrSquarFormState::bombs_amount_string,
            ));

        let make_btn_painter = || {
            Painter::<CustomRectangleOrSquarFormState>::new(|ctx, data, env| {
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
        .on_click(|ctx, form_data: &mut CustomRectangleOrSquarFormState, _| {
            let width = form_data
                .width_string
                .parse::<usize>()
                .unwrap_or(0)
                .min(GAME_MAX_WIDTH);
            let height = form_data
                .height_string
                .parse::<usize>()
                .unwrap_or(0)
                .min(GAME_MAX_HEIGHT);
            let bombs = form_data.bombs_amount_string.parse::<usize>().unwrap_or(0);

            if width >= GAME_MIN_WIDTH && height >= GAME_MIN_HEIGHT && bombs > 0 {
                ctx.submit_command(
                    NEW_GAME_CUSTOM_RECTANGLE_OR_SQUARE.with((width, height, bombs)),
                );
                ctx.submit_command(CLOSE_WINDOW);
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

        let ctrls = Flex::column()
            .with_child(ok_btn)
            .with_spacer(15.0)
            .with_child(cancel_btn);

        Flex::<CustomRectangleOrSquarFormState>::row()
            .with_child(fields)
            .with_spacer(40.0)
            .with_child(ctrls)
            .center()
    }
}
