use druid::{BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId};
use druid::widget::{SizedBox};

pub struct FragmentBox<T> {
    fragment: Box<dyn Widget<T>>,
    f: fn(&T, &Env) -> Box<dyn Widget<T>>,
}

impl<T: Data> FragmentBox<T> {
    pub fn new(f: fn(&T, &Env) -> Box<dyn Widget<T>>) -> Self {
        Self {
            fragment: SizedBox::empty().boxed(),
            f
        }
    }

    fn refresh_fragment(&mut self, data: &T, env: &Env) {
        self.fragment = (self.f)(data, env);
    }
}

impl<T: Data> Widget<T> for FragmentBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.fragment.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.refresh_fragment(data, env);
        }
        self.fragment.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(&data) {
            self.refresh_fragment(data, env);
            ctx.children_changed();
        } else {
            self.fragment.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        self.fragment.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.fragment.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.fragment.id()
    }
}