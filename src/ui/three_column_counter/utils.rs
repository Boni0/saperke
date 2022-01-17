use std::convert::TryInto;
use std::str::FromStr;

use druid::{Widget, WidgetExt, Color, RenderContext};
use druid::widget::{Painter, Svg, SvgData};

use crate::assets::{
    COUNTER_MINUS_SVG_BG,
    COUNTER_NUMS_SVG_BG_ARRAY
};

use crate::consts::{
    TIMER_COLUMN_WIDTH,
    TIMER_COLUMN_HEIGHT
};

pub enum CounterColumn {
    First,
    Second,
    Third 
}

fn get_column_number_svg_str<'a>(num: i64) -> &'a str {
    let idx: usize = num.abs().try_into().unwrap();
    COUNTER_NUMS_SVG_BG_ARRAY[idx]
}

fn get_column_painter(column: CounterColumn) -> Painter<i64> {
    Painter::new(move |ctx, count: &i64, env| {
        let bounds = ctx.size().to_rect();
        ctx.fill(bounds, &Color::BLACK);

        let svg_str: &str = match column {
            CounterColumn::First => {
                if *count < 0 {
                    COUNTER_MINUS_SVG_BG
                } else {
                    get_column_number_svg_str((*count % 1000) / 100)
                }
            },
            CounterColumn::Second => {
                get_column_number_svg_str((*count % 100) / 10)
            },
            CounterColumn::Third => {
                get_column_number_svg_str(*count % 10)
            },
        };

        if let Ok(svg_data) = SvgData::from_str(svg_str) {
            Svg::new(svg_data).paint(ctx, count, env)
        }
    })
}

pub fn get_column_svg(column: CounterColumn) -> impl Widget<i64> {
    Svg::new(SvgData::empty())
        .background(get_column_painter(column))
        .fix_size(TIMER_COLUMN_WIDTH, TIMER_COLUMN_HEIGHT)
}