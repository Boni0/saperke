mod grid;
mod game;
mod renderer;
mod assets;
mod test_console_render;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc, Lens, Data};

use game::Game;
use renderer::GridRenderer;
use assets::SvgAssets;

#[derive(Clone, Data, Lens)]
pub struct AppStruct {
    pub game: Game,
    pub assets: SvgAssets
}

fn build_ui() -> impl Widget<AppStruct> {
    GridRenderer::render()
}

fn main() -> Result<(), PlatformError> {
    let app = AppStruct {
        game: Game::new(),
        assets: SvgAssets::init()
    };

    AppLauncher::with_window(
        WindowDesc
            ::new(build_ui )
            .title("Saperke")
    )
        .launch(app)?;

    Ok(())
}
