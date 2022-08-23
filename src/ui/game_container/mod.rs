mod controllers;

use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::LensWrap;
use druid::{lens, Color, FontFamily, RenderContext, Size, TextAlignment, Widget, WidgetExt};

use controllers::GameContainerController;

use crate::consts::GAME_PAUSED_INFO;
use crate::game::{Game, GameState};

use super::border_box::{BorderBox, BorderColorPattern};
use super::GridWidget;

pub struct GameContainerWidget {
    inner_grid: Box<dyn Widget<Game>>,
}

impl GameContainerWidget {
    pub fn new() -> impl Widget<Game> {
        BorderBox::new(
            GameContainerWidget::with_inner_grid(),
            BorderColorPattern::DarkerFirst,
        )
        .controller(GameContainerController)
    }

    pub fn with_inner_grid() -> Self {
        Self {
            inner_grid: LensWrap::new(GridWidget::new(), lens!(Game, grid)).boxed(),
        }
    }
}

impl Widget<Game> for GameContainerWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Game,
        env: &druid::Env,
    ) {
        self.inner_grid.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &Game,
        env: &druid::Env,
    ) {
        self.inner_grid.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &Game,
        data: &Game,
        env: &druid::Env,
    ) {
        if (old_data.state != GameState::Paused && data.state == GameState::Paused)
            || (old_data.state == GameState::Paused && data.state != GameState::Paused)
        {
            ctx.request_paint();
        }

        self.inner_grid.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &Game,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner_grid.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Game, env: &druid::Env) {
        if data.state == GameState::Paused {
            let rect = ctx.size().to_rect();
            let center = rect.center();

            let text = ctx.text();
            let layout = text
                .new_text_layout(GAME_PAUSED_INFO)
                .font(FontFamily::SYSTEM_UI, 16.0)
                .text_color(Color::BLACK)
                .alignment(TextAlignment::Center)
                .build()
                .unwrap();

            let layout_size: Size = layout.size();

            ctx.draw_text(
                &layout,
                (
                    center.x - (layout_size.width / 2.0),
                    center.y - (layout_size.height / 2.0),
                ),
            );
        } else {
            self.inner_grid.paint(ctx, data, env);
        }
    }
}
