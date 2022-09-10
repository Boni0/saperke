use std::convert::TryInto;
use std::str::FromStr;

use druid::{Widget, WidgetExt, Color, RenderContext};
use druid::widget::{Painter, Svg, SvgData, SizedBox};

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

impl CounterColumn {
    pub fn new(column: CounterColumn, painter: &CounterColumnPainter) -> impl Widget<i64> {
        SizedBox::empty()
            .fix_size(TIMER_COLUMN_WIDTH, TIMER_COLUMN_HEIGHT)
            .background(painter.get(column))
    }
}

pub struct CounterColumnPainter {
    minus_svg: SvgData,
    nums_svg: [SvgData; 10]
}

impl CounterColumnPainter {
    pub fn create() -> Self {
        let svg_data = |str: &str| -> SvgData {
            if let Ok(svg_data) = SvgData::from_str(str) {
                svg_data
            } else {
                SvgData::empty()
            }
        };

        Self {
            minus_svg: svg_data(COUNTER_MINUS_SVG_BG),
            nums_svg: [
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[0]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[1]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[2]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[3]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[4]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[5]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[6]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[7]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[8]),
                svg_data(COUNTER_NUMS_SVG_BG_ARRAY[9]),
            ]
        }
    }

    pub fn get(&self, column: CounterColumn) -> Painter<i64> {
        let minus_svg = self.minus_svg.clone();
        let nums_svg = self.nums_svg.clone();

        let convert_into_usize = |num: i64| -> usize {
            num.abs().try_into().unwrap()
        };

        Painter::new(move |ctx, count: &i64, env| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &Color::BLACK);

            let svg_data = match column {
                CounterColumn::First => {
                    if *count < 0 {
                        &minus_svg
                    } else {
                        &nums_svg[convert_into_usize((*count % 1000) / 100)]
                    }
                },
                CounterColumn::Second => {
                    &nums_svg[convert_into_usize((*count % 100) / 10)]
                },
                CounterColumn::Third => {
                    &nums_svg[convert_into_usize(*count % 10)]
                },
            };
    
            Svg::new(svg_data.clone()).paint(ctx, count, env);
        })
    }
}