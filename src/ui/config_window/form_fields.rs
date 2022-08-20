use druid::{
    widget::{Either, Flex, Label, TextBox},
    Color, Env, Lens, Widget, WidgetExt,
};

use crate::{
    consts::{
        CUSTOM_GAME_BOMBS_FROM_LABEL, CUSTOM_GAME_HEIGHT_FROM_LABEL, CUSTOM_GAME_WIDTH_FROM_LABEL,
    },
    grid::GridStartShape,
};

use super::ConfigWindow;

const LABEL_WIDTH: f64 = 75.0;
const FIELD_WIDTH: f64 = 50.0;
const FIELD_HEIGHT: f64 = 25.0;

pub struct ConfigFormFields;
impl ConfigFormFields {
    fn row_with_label(
        label_txt: &str,
        inner: impl Widget<ConfigWindow> + 'static,
    ) -> impl Widget<ConfigWindow> {
        Flex::<ConfigWindow>::row()
            .with_child(
                Label::new(format!("{}: ", label_txt))
                    .with_text_color(Color::BLACK)
                    .fix_width(LABEL_WIDTH),
            )
            .with_child(inner.boxed().fix_size(FIELD_WIDTH, FIELD_HEIGHT))
    }

    fn size_only_box_shape_textbox<L: Lens<ConfigWindow, String> + Clone + 'static>(
        lens: L,
    ) -> impl Widget<ConfigWindow> {
        let is_box_shape = |config: &ConfigWindow, _: &Env| -> bool {
            match config.custom_start_shape {
                GridStartShape::Box | GridStartShape::PredefinedBox(_) => true,
                _ => false,
            }
        };

        Either::new(
            is_box_shape,
            TextBox::new().lens(lens.clone()),
            Label::new(|value: &String, _env: &_| format!("{}", value))
                .with_text_color(Color::BLACK)
                .lens(lens),
        )
    }

    pub fn new() -> impl Widget<ConfigWindow> {
        let width_field = ConfigFormFields::row_with_label(
            CUSTOM_GAME_WIDTH_FROM_LABEL,
            ConfigFormFields::size_only_box_shape_textbox(ConfigWindow::custom_width),
        );

        let height_field = ConfigFormFields::row_with_label(
            CUSTOM_GAME_HEIGHT_FROM_LABEL,
            ConfigFormFields::size_only_box_shape_textbox(ConfigWindow::custom_height),
        );

        Flex::column()
            .with_child(width_field)
            .with_spacer(10.0)
            .with_child(height_field)
            .with_spacer(10.0)
            .with_child(ConfigFormFields::row_with_label(
                CUSTOM_GAME_BOMBS_FROM_LABEL,
                TextBox::new().lens(ConfigWindow::custom_bombs_amount),
            ))
    }
}
