use druid::widget::{Painter, Svg, SvgData};
use druid::Widget;
use std::str::FromStr;

use crate::assets::{
    FACE_ACTIVE, FACE_IDLE, FACE_LOSS, FACE_WIN, TILE_OPENED_SVG_BG, TILE_UNOPENED_SVG_BG,
};
use crate::game::{Game, GameEndState, GameState};

pub struct FaceButtonPainter {
    tile_opened: SvgData,
    tile_unopened: SvgData,
    face_win: SvgData,
    face_loss: SvgData,
    face_active: SvgData,
    face_idle: SvgData,
}

impl FaceButtonPainter {
    pub fn create() -> Self {
        let svg_data = |str: &str| -> SvgData {
            if let Ok(svg_data) = SvgData::from_str(str) {
                svg_data
            } else {
                SvgData::empty()
            }
        };

        Self { 
            tile_opened: svg_data(TILE_OPENED_SVG_BG), 
            tile_unopened: svg_data(TILE_UNOPENED_SVG_BG), 
            face_win: svg_data(FACE_WIN), 
            face_loss: svg_data(FACE_LOSS), 
            face_active: svg_data(FACE_ACTIVE), 
            face_idle: svg_data(FACE_IDLE), 
        }
    }

    pub fn get_btn(&self) -> Painter<Game> {
        let tile_opened = self.tile_opened.clone();
        let tile_unopened = self.tile_unopened.clone();

        Painter::new(move |ctx, game: &Game, env| {
            let bg_svg = if ctx.is_active() {
                &tile_opened
            } else {
                &tile_unopened
            };

            Svg::new(bg_svg.clone()).paint(ctx, game, env);
        })
    }

    pub fn get_face(&self) -> Painter<Game> {
        let face_win = self.face_win.clone(); 
        let face_loss = self.face_loss.clone();
        let face_active = self.face_active.clone(); 
        let face_idle = self.face_idle.clone(); 

        Painter::new(move |ctx, game: &Game, env| {
            let face_svg = match game.state {
                GameState::EndState(GameEndState::Win) => &face_win,
                GameState::EndState(GameEndState::Loss) => &face_loss,
                _ => {
                    if game.grid.is_active {
                        &face_active
                    } else {
                        &face_idle
                    }
                }
            };

            Svg::new(face_svg.clone()).paint(ctx, game, env);
        })
    }
}