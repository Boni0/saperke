use druid::{Affine, AppLauncher, Color, Env, Event, EventCtx, PlatformError, RenderContext, Widget, WidgetExt, WindowDesc, theme};
use druid::widget::{Label, FillStrat, Flex, FlexParams, CrossAxisAlignment, SizedBox, Painter, Controller, Svg, SvgData};
struct RightClick;

impl<T, W: Widget<T>> Controller<T, W> for RightClick {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // TODO: add right click (bomb) hanler
        child.event(ctx, event, data, env)
    }
}

pub fn create_cell() -> impl Widget<()> {
    let tile_unopened = include_str!("./assets/tiles/unopened.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());
    let mut tile_unopened_svg = Svg::new(tile_unopened);

    let tile_opened = include_str!("./assets/tiles/opened.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());
    let mut tile_opened_svg = Svg::new(tile_opened);

    let num_1 = include_str!("./assets/numbers/1.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());

    let box_painter = Painter::new(move |ctx, _data, env| {
        // let bounds = ctx.size().to_rect();

        tile_unopened_svg.paint(ctx, _data, env);

        if ctx.is_active() {
            tile_opened_svg.paint(ctx, _data, env);
        }
    });

    Svg::new(num_1)
        .fix_size(23.0, 23.0)
        .background(box_painter)
        .controller(RightClick)
        .on_click(|_, _, _| ()) // has to be to update into hot or active
}