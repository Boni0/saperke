use std::time::Duration;

use crate::game::DimensionBombsAmountSettingsTuple;

pub const TITLE: &str = "Saperke";
pub const CUSTOM_GAME_SUBTITLE: &str = "Custom Game";

pub const BACKGROUND_COLOR_HEX: &str = "#B9B9B9";

pub const BORDER_SIZE: f64 = 5.0;
pub const BORDER_LIGHER_HEX: &str = "#FDFCFD";
pub const BORDER_DARKER_HEX: &str = "#757575";

pub const TIMER_COLUMN_WIDTH: f64 = 18.0;
pub const TIMER_COLUMN_HEIGHT: f64 = 32.0;
pub const TIMER_INTERVAL: Duration = Duration::from_millis(10);

pub const GRID_CELL_WIDTH: f64 = 23.0;
pub const GRID_CELL_HEIGHT: f64 = 23.0;

pub const FLEX_COMMON_SPACING_SIZE: f64 = 10.0;

pub const MENU_HEIGHT: f64 = 25.0;

pub const MENU_GAME: &str = "Game";
pub const MENU_GAME_NEW: &str = "New";
pub const MENU_GAME_PAUSE: &str = "Pause";

pub const MENU_GAME_BEGINNER: &str = "Beginner";
pub const MENU_GAME_INTERMEDIATE: &str = "Intermediate";
pub const MENU_GAME_EXPERT: &str = "Expert";
pub const MENU_GAME_CUSTOM: &str = "Custom";

pub const MENU_GAME_ABOUT: &str = "About";
pub const MENU_GAME_EXIT: &str = "Exit";

pub const GAME_MIN_WIDTH: usize = 9;
pub const GAME_MIN_HEIGHT: usize = 9;

pub const GAME_MAX_WIDTH: usize = 30;
pub const GAME_MAX_HEIGHT: usize = 24;

pub const GAME_BEGINNER_DIFFICULTY: DimensionBombsAmountSettingsTuple =
    (GAME_MIN_WIDTH, GAME_MIN_HEIGHT, 10);
pub const GAME_INTERMEDIATE_DIFFICULTY: DimensionBombsAmountSettingsTuple = (16, 16, 40);
pub const GAME_EXPERT_DIFFICULTY: DimensionBombsAmountSettingsTuple = (30, 16, 99);

pub const CUSTOM_GAME_WIDTH_FROM_LABEL: &str = "Width";
pub const CUSTOM_GAME_HEIGHT_FROM_LABEL: &str = "Height";
pub const CUSTOM_GAME_BOMBS_FROM_LABEL: &str = "Mines";

pub const CUSTOM_GAME_CONFIRM_BTN_LABEL: &str = "Ok";
pub const CUSTOM_GAME_CANCEL_BTN_LABEL: &str = "Cancel";
