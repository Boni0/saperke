use std::convert::TryInto;
use std::str::FromStr;

use druid::{Widget, WidgetExt, Color, RenderContext};
use druid::widget::{Painter, Svg, SvgData, Flex};

use crate::assets::{
    COUNTER_MINUS_SVG_BG,
    COUNTER_NUMS_SVG_BG_ARRAY
};

enum CounterColumn {
    First,
    Second,
    Third 
}

pub struct ThreeColumnCounter;

impl ThreeColumnCounter {
    pub fn new() -> impl Widget<i64> {
        let create_brush = |column: CounterColumn| {
            Painter::new(move |ctx, count: &i64, env| {
                let bounds = ctx.size().to_rect();
                ctx.fill(bounds, &Color::BLACK);

                let svg_str: &str = match column {
                    CounterColumn::First => {
                        if *count < 0 {
                            COUNTER_MINUS_SVG_BG
                        } else {
                            let idx: usize = ((*count % 1000) / 100).abs().try_into().unwrap();
                            COUNTER_NUMS_SVG_BG_ARRAY[idx]
                        }
                    },
                    CounterColumn::Second => {
                        let idx: usize = ((*count % 100) / 10).abs().try_into().unwrap();
                        COUNTER_NUMS_SVG_BG_ARRAY[idx]
                    },
                    CounterColumn::Third => {
                        let idx: usize = (*count % 10).abs().try_into().unwrap();
                        COUNTER_NUMS_SVG_BG_ARRAY[idx]
                    },
                };
    
                if let Ok(svg_data) = SvgData::from_str(svg_str) {
                    Svg::new(svg_data).paint(ctx, count, env)
                }
            })
        };

        let mut flex = Flex::row();

        flex.add_child(
            Svg::new(SvgData::empty())
                .background(create_brush(CounterColumn::First))
                .fix_size(18.0, 32.0)
        );

        flex.add_child(
            Svg::new(SvgData::empty())
                .background(create_brush(CounterColumn::Second))
                .fix_size(18.0, 32.0)
        );

        flex.add_child(
            Svg::new(SvgData::empty())
                .background(create_brush(CounterColumn::Third))
                .fix_size(18.0, 32.0)
        );

        flex
    }
}