use crate::grid::{GridShapeSizeUnit, GridSizeUnit, NonExistedPoints};

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
    pub fn get_predefined_unusual() {
        // GameDifficultyGrid::UnusualRandom(())
    }
}
