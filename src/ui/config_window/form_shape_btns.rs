use druid::{
    widget::{Flex, Label, Painter, SizedBox},
    Color, Widget, WidgetExt,
};

use crate::{
    grid::{GridPredefinedBoxDifficulty, GridStartShape, GridUnusualVariant},
    ui::border_box::{BorderBox, BorderColorPattern},
};
use strum::IntoEnumIterator;

use super::ConfigWindow;

pub struct ConfigWindowShapeBtns;

impl ConfigWindowShapeBtns {
    pub fn new() -> impl Widget<ConfigWindow> {
        let make_btn_painter = |shapes_to_check: Vec<GridStartShape>| {
            Painter::<ConfigWindow>::new(move |ctx, data, env| {
                let pattern =
                    if ctx.is_active() || shapes_to_check.contains(&data.custom_start_shape) {
                        BorderColorPattern::DarkerFirst
                    } else {
                        BorderColorPattern::LigherFirst
                    };

                BorderBox::new_with_custom_size(SizedBox::empty(), pattern, 2.0)
                    .paint(ctx, data, env);
            })
        };

        let mut main_row = Flex::row();

        main_row.add_child(
            SizedBox::new(Label::new("Box").with_text_color(Color::BLACK).center())
                .background(make_btn_painter(vec![
                    GridStartShape::Box,
                    GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Beginner),
                    GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Intermediate),
                    GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Expert),
                ]))
                .fix_size(60.0, 25.0)
                .on_click(|_, data: &mut ConfigWindow, _| {
                    if let GridStartShape::Unusual(_) = data.custom_start_shape {
                        data.custom_start_shape = GridStartShape::Box
                    }
                }),
        );

        for variant in GridUnusualVariant::iter() {
            let size = GridUnusualVariant::get_variant_size(&variant);

            main_row.add_default_spacer();
            main_row.add_child(
                SizedBox::new(
                    Label::new(GridUnusualVariant::get_variant_label(&variant))
                        .with_text_color(Color::BLACK)
                        .center(),
                )
                .background(make_btn_painter(vec![GridStartShape::Unusual(
                    variant.clone(),
                )]))
                .fix_size(60.0, 25.0)
                .on_click(move |_, data: &mut ConfigWindow, _| {
                    if data.custom_start_shape != GridStartShape::Unusual(variant.clone()) {
                        data.custom_width = size.width.to_string();
                        data.custom_height = size.height.to_string();
                        data.custom_start_shape = GridStartShape::Unusual(variant.clone());
                    }
                }),
            );
        }

        main_row
    }
}
