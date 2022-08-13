use druid::{Data, Lens};

use crate::game::Game;

#[derive(Clone, Data, Lens)]
pub struct CustomRectangleOrSquarFormState {
    game: Game,
    pub width_string: String,
    pub height_string: String,
    pub bombs_amount_string: String,
}

impl CustomRectangleOrSquarFormState {
    pub fn new(game: Game) -> Self {
        let width_string = game.grid.size.width.to_string();
        let height_string = game.grid.size.height.to_string();
        let bombs_amount_string = game.grid.bombs.count.to_string();

        Self {
            game,
            width_string,
            height_string,
            bombs_amount_string,
        }
    }
}
