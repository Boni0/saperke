use druid::{LocalizedString, MenuDesc, MenuItem};

use crate::{
    app::AppState,
    delegate::{NEW_GAME_STANDARD, RESTART_GAME},
    game::StandardGameDifficulty,
};

pub fn create_app_menu() -> MenuDesc<AppState> {
    let menu = MenuDesc::<AppState>::empty().append(
        MenuDesc::new(LocalizedString::new("Game"))
            .append(MenuItem::new(LocalizedString::new("New"), RESTART_GAME))
            .append_separator()
            .append(MenuItem::new(
                LocalizedString::new("Beginner"),
                NEW_GAME_STANDARD.with(StandardGameDifficulty::Beginner),
            ))
            .append(MenuItem::new(
                LocalizedString::new("Intermediate"),
                NEW_GAME_STANDARD.with(StandardGameDifficulty::Intermediate),
            ))
            .append(MenuItem::new(
                LocalizedString::new("Expert"),
                NEW_GAME_STANDARD.with(StandardGameDifficulty::Expert),
            ))
            // .append_separator()
            // .append(MenuItem::new(LocalizedString::new("Exi1t"), QUIT_APP)),
    );

    menu
}
