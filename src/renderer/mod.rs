use druid::{Env, Event, EventCtx, Widget, WidgetExt};
use druid::widget::{Painter, Controller, Svg, SvgData};

use crate::grid::{GridCell, GridCellState};
mod fragment;

pub use fragment::{FragmentBox};

struct RightClick;
impl<T, W: Widget<T>> Controller<T, W> for RightClick {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // TODO: add right click (bomb) hanler
        child.event(ctx, event, data, env)
    }
}

pub fn create_cell(cell: &GridCell) -> Box<dyn Widget<GridCell>> {
    let tile_unopened = include_str!("../assets/tiles/unopened.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());
    let tile_unopened_svg = Svg::new(tile_unopened);

    let tile_opened = include_str!("../assets/tiles/opened.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());
    let tile_opened_svg = Svg::new(tile_opened);

    let mut active_tile_svg = match cell.state {
        GridCellState::Hidden => tile_unopened_svg,
        _ => tile_opened_svg,
    };

    let _num_1 = include_str!("../assets/numbers/1.svg")
        .parse::<SvgData>()
        .unwrap_or(SvgData::default());

    let box_painter = Painter::new(move |ctx, _data, env| {
        active_tile_svg.paint(ctx, _data, env);
    }); 

    Svg::new(SvgData::empty())
        .lens(GridCell::state)
        .fix_size(23.0, 23.0)
        .background(box_painter)
        .controller(RightClick)
        .on_click(|_, _, _| ()) // has to be to update into hot or active
        .boxed()
}