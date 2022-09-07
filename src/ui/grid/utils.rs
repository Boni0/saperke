use druid::widget::SvgData;
use std::str::FromStr;

use crate::assets::{
    BOMB_SIGN_SVG_BG, FLAG_SIGN_SVG_BG, NUMS_SVG_BG_ARRAY, QUESTION_MARK_SIGN_SVG_BG,
    TILE_OPENED_SVG_BG, TILE_UNOPENED_SVG_BG, X_SIGN_SVG_BG,
};

#[derive(Clone)]
pub struct PainterSvgData {
    pub tile_unopened: SvgData,
    pub tile_opened: SvgData,
    pub question_mark: SvgData,
    pub flag_sign: SvgData,
    pub nums: [SvgData; 9],
    pub bomb: SvgData,
    pub x_sign: SvgData,
}

pub fn init_cell_painter() -> PainterSvgData {
    let svg_data = |str: &str| -> SvgData {
        if let Ok(svg_data) = SvgData::from_str(str) {
            svg_data
        } else {
            SvgData::empty()
        }
    };

    let tile_unopened = svg_data(TILE_UNOPENED_SVG_BG);
    let tile_opened = svg_data(TILE_OPENED_SVG_BG);

    let question_mark = svg_data(QUESTION_MARK_SIGN_SVG_BG);
    let flag_sign = svg_data(FLAG_SIGN_SVG_BG);

    let nums = [
        svg_data(NUMS_SVG_BG_ARRAY[0]),
        svg_data(NUMS_SVG_BG_ARRAY[1]),
        svg_data(NUMS_SVG_BG_ARRAY[2]),
        svg_data(NUMS_SVG_BG_ARRAY[3]),
        svg_data(NUMS_SVG_BG_ARRAY[4]),
        svg_data(NUMS_SVG_BG_ARRAY[5]),
        svg_data(NUMS_SVG_BG_ARRAY[6]),
        svg_data(NUMS_SVG_BG_ARRAY[7]),
        svg_data(NUMS_SVG_BG_ARRAY[8]),
    ];

    let bomb = svg_data(BOMB_SIGN_SVG_BG);
    let x_sign = svg_data(X_SIGN_SVG_BG);

    PainterSvgData {
        tile_unopened,
        tile_opened,
        question_mark,
        flag_sign,
        nums,
        bomb,
        x_sign,
    }
}
