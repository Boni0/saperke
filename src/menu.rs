use druid::{commands::QUIT_APP, LocalizedString, MenuDesc, MenuItem};

use crate::{
    app::AppState,
    consts::{
        MENU_GAME, MENU_GAME_BEGINNER, MENU_GAME_CUSTOM, MENU_GAME_EXIT, MENU_GAME_EXPERT,
        MENU_GAME_INTERMEDIATE, MENU_GAME_NEW,
    },
    delegate::{NEW_GAME_PREDEFINED_BOX, OPEN_CUSTOM_GAME_WINDOW, RESTART_GAME},
    grid::GridPredefinedBoxDifficulty,
};

pub fn create_app_menu() -> MenuDesc<AppState> {
    let menu = MenuDesc::<AppState>::empty().append(
        MenuDesc::new(LocalizedString::new(MENU_GAME))
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_NEW),
                RESTART_GAME,
            ))
            .append_separator()
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_BEGINNER),
                NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Beginner),
            ))
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_INTERMEDIATE),
                NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Intermediate),
            ))
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_EXPERT),
                NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Expert),
            ))
            .append_separator()
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_CUSTOM),
                OPEN_CUSTOM_GAME_WINDOW,
            ))
            .append_separator()
            .append(MenuItem::new(
                LocalizedString::new(MENU_GAME_EXIT),
                QUIT_APP,
            )),
    );

    menu
}
