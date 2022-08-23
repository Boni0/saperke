use druid::{commands::QUIT_APP, LocalizedString, MenuDesc, MenuItem, SysMods};

use crate::{
    app::AppState,
    consts::{
        MENU_GAME, MENU_GAME_BEGINNER, MENU_GAME_CUSTOM, MENU_GAME_EXIT, MENU_GAME_EXPERT,
        MENU_GAME_INTERMEDIATE, MENU_GAME_NEW, MENU_GAME_PAUSE,
    },
    delegate::{NEW_GAME_PREDEFINED_BOX, OPEN_CUSTOM_GAME_WINDOW, RESTART_GAME, TOGGLE_PAUSE_GAME},
    game::GameState,
    grid::{GridPredefinedBoxDifficulty, GridStartShape},
};

pub fn create_app_menu(app: &AppState) -> MenuDesc<AppState> {
    let menu = MenuDesc::<AppState>::empty().append(
        MenuDesc::new(LocalizedString::new(MENU_GAME))
            .append(
                MenuItem::new(LocalizedString::new(MENU_GAME_NEW), RESTART_GAME)
                    .hotkey(SysMods::Cmd, "r"),
            )
            .append(
                MenuItem::new(LocalizedString::new(MENU_GAME_PAUSE), TOGGLE_PAUSE_GAME)
                    .hotkey(SysMods::Cmd, "p")
                    .disabled_if(|| {
                        app.game.state != GameState::Paused && app.game.state != GameState::Running
                    }),
            )
            .append_separator()
            .append(
                MenuItem::new(
                    LocalizedString::new(MENU_GAME_BEGINNER),
                    NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Beginner),
                )
                .selected_if(|| {
                    app.game.grid.start_shape
                        == GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Beginner)
                })
                .hotkey(SysMods::Cmd, "1"),
            )
            .append(
                MenuItem::new(
                    LocalizedString::new(MENU_GAME_INTERMEDIATE),
                    NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Intermediate),
                )
                .selected_if(|| {
                    app.game.grid.start_shape
                        == GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Intermediate)
                })
                .hotkey(SysMods::Cmd, "2"),
            )
            .append(
                MenuItem::new(
                    LocalizedString::new(MENU_GAME_EXPERT),
                    NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Expert),
                )
                .selected_if(|| {
                    app.game.grid.start_shape
                        == GridStartShape::PredefinedBox(GridPredefinedBoxDifficulty::Expert)
                })
                .hotkey(SysMods::Cmd, "3"),
            )
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
