use druid::{im::Vector, Data};
use strum_macros::EnumIter;

use crate::{
    consts::{
        CUSTOM_GAME_SHAPE_CIRCLE_NAME, CUSTOM_GAME_SHAPE_HEART_NAME, CUSTOM_GAME_SHAPE_HEHE_NAME,
    },
    variants::{
        CIRCLE_EMPTY_POINTS, CIRCLE_SIZE, HEART_EMPTY_POINTS, HEART_SIZE, HEHE_EMPTY_POINTS,
        HEHE_SIZE,
    },
};

use super::{GridCellPoint, GridSize, NonExistedPoints, SizeUnit};

#[derive(Clone, Data, PartialEq, EnumIter)]
pub enum GridUnusualVariant {
    Heart,
    Hehe,
    Circle,
}

impl GridUnusualVariant {
    pub fn get_variant_size(variant: &GridUnusualVariant) -> GridSize {
        match variant {
            GridUnusualVariant::Heart => HEART_SIZE,
            GridUnusualVariant::Hehe => HEHE_SIZE,
            GridUnusualVariant::Circle => CIRCLE_SIZE,
        }
    }

    pub fn get_variant_non_existed_points(variant: &GridUnusualVariant) -> NonExistedPoints {
        match variant {
            GridUnusualVariant::Heart => {
                GridUnusualVariant::convert_array_to_non_existed_points(&(HEART_EMPTY_POINTS))
            }
            GridUnusualVariant::Hehe => {
                GridUnusualVariant::convert_array_to_non_existed_points(&(HEHE_EMPTY_POINTS))
            }
            GridUnusualVariant::Circle => {
                GridUnusualVariant::convert_array_to_non_existed_points(&(CIRCLE_EMPTY_POINTS))
            }
        }
    }

    pub fn get_variant_label<'a>(variant: &'a GridUnusualVariant) -> &'a str {
        match variant {
            GridUnusualVariant::Heart => CUSTOM_GAME_SHAPE_HEART_NAME,
            GridUnusualVariant::Hehe => CUSTOM_GAME_SHAPE_HEHE_NAME,
            GridUnusualVariant::Circle => CUSTOM_GAME_SHAPE_CIRCLE_NAME,
        }
    }

    pub fn get_variant_data<'a>(
        variant: &'a GridUnusualVariant,
    ) -> (GridSize, NonExistedPoints, &'a str) {
        (
            GridUnusualVariant::get_variant_size(variant),
            GridUnusualVariant::get_variant_non_existed_points(variant),
            GridUnusualVariant::get_variant_label(variant),
        )
    }

    fn convert_array_to_non_existed_points(start_arr: &[(SizeUnit, SizeUnit)]) -> NonExistedPoints {
        let mut vec: NonExistedPoints = Vector::new();

        for (y, x) in start_arr {
            vec.push_back(GridCellPoint { x: *x, y: *y })
        }

        vec
    }
}

#[derive(Clone, Data, PartialEq)]
pub enum GridPredefinedBoxDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Clone, Data, PartialEq)]
pub enum GridStartShape {
    Box,
    PredefinedBox(GridPredefinedBoxDifficulty),
    Unusual(GridUnusualVariant),
}
