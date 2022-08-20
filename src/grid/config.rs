use druid::im::Vector;
use druid::{Data, Lens};

use crate::{
    consts::{
        GAME_BEGINNER_DIFFICULTY_BOMBS_AMOUNT, GAME_BEGINNER_DIFFICULTY_SIZE,
        GAME_EXPERT_DIFFICULTY_BOMBS_AMOUNT, GAME_EXPERT_DIFFICULTY_SIZE,
        GAME_INTERMEDIATE_DIFFICULTY_BOMBS_AMOUNT, GAME_INTERMEDIATE_DIFFICULTY_SIZE,
    },
    unusual_predefined::{HEART_EMPTY_POINTS, HEART_SIZE},
};

use super::*;

pub type SizeUnit = usize;

#[derive(Clone, Lens, Data)]
pub struct GridConfig {
    pub size: GridSize,
    pub start_shape: GridStartShape,
    pub non_existed_points: Option<NonExistedPoints>,
    pub bombs_config: GridBombsConfig,
}

impl GridConfig {
    pub fn predefined_box(difficulty: GridPredefinedBoxDifficulty) -> Self {
        let (size, bombs_amount) = match difficulty {
            GridPredefinedBoxDifficulty::Beginner => (
                GAME_BEGINNER_DIFFICULTY_SIZE,
                GAME_BEGINNER_DIFFICULTY_BOMBS_AMOUNT,
            ),
            GridPredefinedBoxDifficulty::Intermediate => (
                GAME_INTERMEDIATE_DIFFICULTY_SIZE,
                GAME_INTERMEDIATE_DIFFICULTY_BOMBS_AMOUNT,
            ),
            GridPredefinedBoxDifficulty::Expert => (
                GAME_EXPERT_DIFFICULTY_SIZE,
                GAME_EXPERT_DIFFICULTY_BOMBS_AMOUNT,
            ),
        };

        Self {
            size,
            start_shape: GridStartShape::PredefinedBox(difficulty),
            non_existed_points: None,
            bombs_config: GridBombsConfig::Randomized(bombs_amount),
        }
    }

    pub fn custom_box(
        size: GridSize,
        bombs_config: GridBombsConfig,
        non_existed_points: Option<NonExistedPoints>,
    ) -> Self {
        Self {
            size,
            start_shape: GridStartShape::Box,
            non_existed_points,
            bombs_config,
        }
    }

    pub fn simple_custom_box(size: GridSize, bombs_amount: SizeUnit) -> Self {
        GridConfig::custom_box(size, GridBombsConfig::Randomized(bombs_amount), None)
    }

    pub fn unusual(
        variant: GridUnusualVariant,
        bombs_config: GridBombsConfig,
        extra_non_existed_points: Option<NonExistedPoints>,
    ) -> Self {
        let (size, mut non_existed_points) = match variant {
            GridUnusualVariant::Heart => (
                HEART_SIZE,
                GridConfig::convert_array_to_non_existed_points(&HEART_EMPTY_POINTS),
            ),
        };

        if let Some(extra_points) = extra_non_existed_points {
            non_existed_points.append(extra_points);
        }

        Self {
            size,
            start_shape: GridStartShape::Unusual(variant),
            non_existed_points: Some(non_existed_points),
            bombs_config,
        }
    }

    pub fn simple_unusual(variant: GridUnusualVariant, bombs_amount: SizeUnit) -> Self {
        GridConfig::unusual(variant, GridBombsConfig::Randomized(bombs_amount), None)
    }

    fn convert_array_to_non_existed_points(start_arr: &[(SizeUnit, SizeUnit)]) -> NonExistedPoints {
        let mut vec: NonExistedPoints = Vector::new();

        for (y, x) in start_arr {
            vec.push_back(GridCellPoint { x: *x, y: *y })
        }

        vec
    }
}
