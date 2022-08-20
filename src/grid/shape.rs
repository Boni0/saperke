use druid::{im::Vector, Data};
use strum_macros::EnumIter;

use crate::unusual_predefined::{HEART_EMPTY_POINTS, HEART_SIZE};

use super::{GridCellPoint, GridSize, NonExistedPoints, SizeUnit};

#[derive(Clone, Data, PartialEq, EnumIter)]
pub enum GridUnusualVariant {
    Heart,
}

impl GridUnusualVariant {
    pub fn get_variant_size(variant: &GridUnusualVariant) -> GridSize {
        match variant {
            GridUnusualVariant::Heart => HEART_SIZE,
        }
    }

    pub fn get_variant_non_existed_points(variant: &GridUnusualVariant) -> NonExistedPoints {
        GridUnusualVariant::convert_array_to_non_existed_points(
            &(match variant {
                GridUnusualVariant::Heart => HEART_EMPTY_POINTS,
            }),
        )
    }

    pub fn get_variant_label<'a>(variant: &'a GridUnusualVariant) -> &'a str {
        match variant {
            GridUnusualVariant::Heart => "Heart",
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
