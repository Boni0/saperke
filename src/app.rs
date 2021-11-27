use druid::{Data, Lens};

use crate::game::Game;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub game: Game,
}

