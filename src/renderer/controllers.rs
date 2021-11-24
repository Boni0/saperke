use druid::{Env, Event, EventCtx, Widget};
use druid::widget::{Controller};

pub struct ClickController;
impl<T, W: Widget<T>> Controller<T, W> for ClickController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // TODO: add right click (bomb) hanler
        child.event(ctx, event, data, env)
    }
}