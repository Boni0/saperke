use druid::Data;

use crate::grid::{GridShapeSizeUnit, GridSizeUnit, NonExistedPoints};

pub type DimensionBombsAmountSettingsTuple = (GridShapeSizeUnit, GridShapeSizeUnit, GridSizeUnit);
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
    pub fn get_predefined_unusual() {
        // GameDifficultyGrid::UnusualRandom(())
    }
}
