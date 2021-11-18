mod grid;
mod game;
mod renderer;
mod test_console_render;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use druid::widget::{Flex};

fn build_ui() -> impl Widget<()> {
    Flex::row().with_child(
        renderer::create_cell()
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui )).launch(())?;
    Ok(())
}
