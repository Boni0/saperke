use std::str::FromStr;

use druid::{Widget, WidgetExt, lens, Color, RenderContext};
use druid::widget::{Button, Flex, Svg, SvgData, Painter};

use crate::app::AppState;
use crate::game::{Game, GameState};

use crate::assets::{
    COUNTER_NUMS_SVG_BG_ARRAY,
    COUNTER_MINUS_SVG_BG
};

pub struct InfoPanel;

impl InfoPanel {
    pub fn new() -> impl Widget<AppState> {
        let mut flex = Flex::row();


        let brush = Painter::new(|ctx, app_data: &AppState, env| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &Color::BLACK);

            // if let Ok(svg_data) = SvgData::from_str(COUNTER_NONE_SVG_BG) {
            //     Svg::new(svg_data).paint(ctx, app_data, env)
            // }

            if let Ok(svg_data) = SvgData::from_str(COUNTER_MINUS_SVG_BG) {
                Svg::new(svg_data).paint(ctx, app_data, env)
            }
        });

        flex.add_child(
            Svg::new(SvgData::empty())
                .background(brush)
                .fix_size(18.0, 32.0)
                // .fix_size(88.0, 164.0)
        );

        flex.add_child(
            Button::new("New")
            .on_click(|_, game: &mut Game, _| {
                game.grid.refresh();
                game.state = GameState::Running;
            })
            .lens(lens!(AppState, game))
        );

        flex
    }
}