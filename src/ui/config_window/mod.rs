mod form_action_btns;
mod form_fields;

use druid::{widget::Flex, Data, Lens, Widget, WidgetExt};

use crate::grid::{Grid, GridStartShape};

use form_action_btns::ConfigWindowActionBtns;
use form_fields::ConfigFormFields;

#[derive(Clone, Data, Lens)]
pub struct ConfigWindow {
    pub game_grid: Grid,
    pub custom_start_shape: GridStartShape,
    pub custom_width: String,
    pub custom_height: String,
    pub custom_bombs_amount: String,
}

impl ConfigWindow {
    pub fn create_state(game_grid: Grid) -> ConfigWindow {
        let custom_width = game_grid.size.width.to_string();
        let custom_height = game_grid.size.height.to_string();
        let custom_bombs_amount = game_grid.bombs.count.to_string();

        ConfigWindow {
            game_grid: game_grid.clone(),
            custom_start_shape: game_grid.start_shape,
            custom_width,
            custom_height,
            custom_bombs_amount,
        }
    }

    pub fn new() -> impl Widget<ConfigWindow> {
        Flex::<ConfigWindow>::row()
            .with_child(ConfigFormFields::new())
            .with_spacer(40.0)
            .with_child(ConfigWindowActionBtns::new())
            .center()
    }
}
