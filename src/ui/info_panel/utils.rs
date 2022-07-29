use druid::widget::{Painter, Svg, SvgData};
use druid::Widget;
use std::str::FromStr;

use crate::assets::{
    FACE_ACTIVE, FACE_IDLE, FACE_LOSS, FACE_WIN, TILE_OPENED_SVG_BG, TILE_UNOPENED_SVG_BG,
};
use crate::game::{Game, GameEndState, GameState};
use crate::grid::GridCellState;

pub fn get_btn_painter() -> Painter<Game> {
    Painter::new(move |ctx, game: &Game, env| {
        if let Ok(bg_svg) = SvgData::from_str(if ctx.is_active() {
            TILE_OPENED_SVG_BG
        } else {
            TILE_UNOPENED_SVG_BG
        }) {
            Svg::new(bg_svg).paint(ctx, game, env);
        }

        if let Ok(face_svg) = SvgData::from_str(match game.state {
            GameState::EndState(GameEndState::Win) => FACE_WIN,
            GameState::EndState(GameEndState::Loss) => FACE_LOSS,
            _ => {
                if game.grid.is_active {
                    FACE_ACTIVE
                } else {
                    FACE_IDLE
                }
            }
        }) {
            Svg::new(face_svg).paint(ctx, game, env);
        }
    })
}
