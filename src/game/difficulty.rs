use druid::im::Vector;

use crate::{
    grid::{GridCellPoint, GridShapeSizeUnit, GridSizeUnit, NonExistedPoints},
    unusual_predefined::{HEART_EMPTY_POINTS, HEART_SHAPE_HEIGHT, HEART_SHAPE_WIDTH},
};

// (Width, Height, Mines)
pub type DimensionBombsAmountSettingsTuple = (GridShapeSizeUnit, GridShapeSizeUnit, GridSizeUnit);

// (Width, Height, Empty Points Vec, Mines)
pub type DimensionNonExistedPointsBombsAmountSettingsTuple = (
    GridShapeSizeUnit,
    GridShapeSizeUnit,
    NonExistedPoints,
    GridSizeUnit,
);

#[derive(Clone)]
pub enum StandardGameDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

pub enum GameDifficultyGrid {
    Standard(StandardGameDifficulty),
    CustomRectangleOrSquareRandom(DimensionBombsAmountSettingsTuple),
    UnusualRandom(DimensionNonExistedPointsBombsAmountSettingsTuple),
}

impl GameDifficultyGrid {
    fn predefined_to_non_existed_points_vec(
        start_arr: &[(GridSizeUnit, GridSizeUnit)],
    ) -> NonExistedPoints {
        let mut vec: NonExistedPoints = Vector::new();

        for (y, x) in start_arr {
            vec.push_back(GridCellPoint { x: *x, y: *y })
        }

        vec
    }

    pub fn get_heart() -> GameDifficultyGrid {
        GameDifficultyGrid::UnusualRandom((
            HEART_SHAPE_WIDTH,
            HEART_SHAPE_HEIGHT,
            GameDifficultyGrid::predefined_to_non_existed_points_vec(&HEART_EMPTY_POINTS),
            10,
        ))
    }
}
