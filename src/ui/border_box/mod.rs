use crate::consts::{BORDER_DARKER_HEX, BORDER_LIGHER_HEX, BORDER_SIZE};
use druid::kurbo::BezPath;
use druid::{
    Color, Data, Env, LifeCycle, LifeCycleCtx, RenderContext, Size, UpdateCtx, Widget, WidgetExt,
};

#[derive(PartialEq)]
pub enum BorderColorPattern {
    LigherFirst,
    DarkerFirst,
}

pub struct BorderBox<'a, T: Data> {
    inner: Box<dyn Widget<T>>,
    border_colors: (&'a str, &'a str),
    border_size: f64,
}

impl<'a, T: Data> BorderBox<'a, T> {
    pub fn new(inner: impl Widget<T> + 'static, color_pattern: BorderColorPattern) -> Self {
        BorderBox::new_with_custom_size(inner, color_pattern, BORDER_SIZE)
    }

    pub fn new_with_custom_size(
        inner: impl Widget<T> + 'static,
        color_pattern: BorderColorPattern,
        border_size: f64,
    ) -> Self {
        let mut border_colors = (BORDER_LIGHER_HEX, BORDER_DARKER_HEX);
        if color_pattern == BorderColorPattern::DarkerFirst {
            border_colors = (border_colors.1, border_colors.0)
        }

        Self {
            inner: inner.padding(border_size).boxed(),
            border_colors,
            border_size,
        }
    }
}

impl<'a, T: Data> Widget<T> for BorderBox<'a, T> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();

        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((size.width, 0.0));
        path.line_to((size.width - self.border_size, self.border_size));
        path.line_to((self.border_size, self.border_size));
        path.close_path();
        ctx.fill(path, &Color::from_hex_str(self.border_colors.0).unwrap());

        let mut path = BezPath::new();
        path.move_to((0.0, 0.0));
        path.line_to((0.0, size.height));
        path.line_to((self.border_size, size.height - self.border_size));
        path.line_to((self.border_size, self.border_size));
        path.close_path();
        ctx.fill(path, &Color::from_hex_str(self.border_colors.0).unwrap());

        let mut path = BezPath::new();
        path.move_to((size.width, 0.0));
        path.line_to((size.width, size.height));
        path.line_to((
            size.width - self.border_size,
            size.height - self.border_size,
        ));
        path.line_to((size.width - self.border_size, self.border_size));
        path.close_path();
        ctx.fill(path, &Color::from_hex_str(self.border_colors.1).unwrap());

        let mut path = BezPath::new();
        path.move_to((0.0, size.height));
        path.line_to((size.width, size.height));
        path.line_to((
            size.width - self.border_size,
            size.height - self.border_size,
        ));
        path.line_to((self.border_size, size.height - self.border_size));
        path.close_path();
        ctx.fill(path, &Color::from_hex_str(self.border_colors.1).unwrap());

        self.inner.paint(ctx, data, env);
    }
}
