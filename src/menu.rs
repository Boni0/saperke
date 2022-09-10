use druid::{commands::QUIT_APP, Env, LocalizedString, Menu, MenuItem, SysMods, WindowId};

use crate::{
    app::AppState,
    consts::{
        MENU_GAME, MENU_GAME_ABOUT, MENU_GAME_BEGINNER, MENU_GAME_CUSTOM, MENU_GAME_EXIT,
        MENU_GAME_EXPERT, MENU_GAME_INTERMEDIATE, MENU_GAME_NEW, MENU_GAME_PAUSE,
    },
    delegate::{
        NEW_GAME_PREDEFINED_BOX, OPEN_ABOUT_WINDOW, OPEN_CUSTOM_GAME_WINDOW, RESTART_GAME,
        TOGGLE_PAUSE_GAME,
    },
    game::GameState,
    grid::{GridPredefinedBoxDifficulty, GridStartShape},
};

pub fn create_app_menu(_: Option<WindowId>, _: &AppState, _: &Env) -> Menu<AppState> {
    let menu = Menu::empty()
        .entry(
            Menu::new(LocalizedString::new(MENU_GAME))
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_NEW))
                        .command(RESTART_GAME)
                        .hotkey(SysMods::Cmd, "r"),
                )
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_PAUSE))
                        .command(TOGGLE_PAUSE_GAME)
                        .hotkey(SysMods::Cmd, "p")
                        .enabled_if(|app: &AppState, _| {
                            app.game.state == GameState::Paused
                                || app.game.state == GameState::Running
                        }),
                )
                .separator()
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_BEGINNER))
                        .command(
                            NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Beginner),
                        )
                        .selected_if(|app: &AppState, _| {
                            app.game.grid.start_shape
                                == GridStartShape::PredefinedBox(
                                    GridPredefinedBoxDifficulty::Beginner,
                                )
                        })
                        .hotkey(SysMods::Cmd, "1"),
                )
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_INTERMEDIATE))
                        .command(
                            NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Intermediate),
                        )
                        .selected_if(|app: &AppState, _| {
                            app.game.grid.start_shape
                                == GridStartShape::PredefinedBox(
                                    GridPredefinedBoxDifficulty::Intermediate,
                                )
                        })
                        .hotkey(SysMods::Cmd, "2"),
                )
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_EXPERT))
                        .command(NEW_GAME_PREDEFINED_BOX.with(GridPredefinedBoxDifficulty::Expert))
                        .selected_if(|app: &AppState, _| {
                            app.game.grid.start_shape
                                == GridStartShape::PredefinedBox(
                                    GridPredefinedBoxDifficulty::Expert,
                                )
                        })
                        .hotkey(SysMods::Cmd, "3"),
                )
                .separator()
                .entry(
                    MenuItem::new(LocalizedString::new(MENU_GAME_CUSTOM))
                        .command(OPEN_CUSTOM_GAME_WINDOW)
                        .hotkey(SysMods::Cmd, "4"),
                )
                .separator()
                .entry(MenuItem::new(LocalizedString::new(MENU_GAME_ABOUT)).command(OPEN_ABOUT_WINDOW))
                .entry(MenuItem::new(LocalizedString::new(MENU_GAME_EXIT)).command(QUIT_APP).hotkey(SysMods::Cmd, "q"))
        );
        
    menu
}
