mod grid;
mod game;
mod renderer;
mod assets;
// mod test_console_render;

use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, Lens, Data};
use druid::widget::Flex;

use game::Game;
use renderer::GridRenderer;

#[derive(Clone, Data, Lens)]
pub struct AppStruct {
    pub game: Game,
}

fn build_ui() -> impl Widget<AppStruct> {
    let mut flex = Flex::column();
    flex.add_child(GridRenderer::render());
    flex.center()
}

fn main() -> Result<(), PlatformError> {
    let app = AppStruct {
        game: Game::new(),
    };

    AppLauncher::with_window(
        WindowDesc
            ::new(build_ui )
            .title("Saperke")
    )
        .launch(app)?;

    Ok(())
}
