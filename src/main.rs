mod grid;
mod game;
mod renderer;
mod test_console_render;

use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::{List, SizedBox};

use game::{GameStruct};
use grid::{GridStruct};

use renderer::{FragmentBox};

fn build_ui() -> impl Widget<GameStruct> {
    SizedBox::new(
        List::new(|| {
            List::new(|| {
                FragmentBox::new(|cell, _| {
                    renderer::create_cell(cell)
                })
            })
            .horizontal()
        })
        .lens(GridStruct::cells)
    )
    .lens(GameStruct::grid)
}

fn main() -> Result<(), PlatformError> {
    let game = GameStruct::new();

    AppLauncher::with_window(
        WindowDesc::new(build_ui )
            .title("Saperke")
    )
        .launch(game)?;
    Ok(())
}
