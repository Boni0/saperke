mod form_action_btns;
mod form_fields;
mod form_shape_btns;

use druid::{widget::Flex, Data, Lens, Size, Widget, WidgetExt};

use crate::grid::{Grid, GridStartShape};

use form_action_btns::ConfigWindowActionBtns;
use form_fields::ConfigFormFields;

use self::form_shape_btns::ConfigWindowShapeBtns;

pub const CONFIG_WINDOW_SIZE: Size = Size {
    width: 350.0,
    height: 250.0,
};

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
        let form = Flex::<ConfigWindow>::row()
            .with_child(ConfigFormFields::new())
            .with_spacer(20.0)
            .with_child(ConfigWindowActionBtns::new())
            .center();

        Flex::<ConfigWindow>::column()
            .with_child(ConfigWindowShapeBtns::new())
            .with_spacer(20.0)
            .with_child(form)
            .center()
    }
}
