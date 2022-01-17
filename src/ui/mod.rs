mod grid;
mod info_panel;
mod three_column_counter;

use druid::{Widget, WidgetExt};
use druid::widget::Flex;
use crate::AppState;

pub use grid::GridWidget;
pub use info_panel::InfoPanel;
pub use three_column_counter::ThreeColumnCounter;

pub fn build() -> impl Widget<AppState> {
    let mut flex = Flex::column();
    flex.add_child(InfoPanel::new());
    flex.add_child(GridWidget::new());
    flex.center()
}