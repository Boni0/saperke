use druid::Data;

#[derive(Clone, Data, PartialEq)]
pub enum GridUnusualVariant {
    Heart,
}

#[derive(Clone, Data, PartialEq)]
pub enum GridPredefinedBoxDifficulty {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Clone, Data)]
pub enum GridStartShape {
    Box,
    PredefinedBox(GridPredefinedBoxDifficulty),
    Unusual(GridUnusualVariant),
}
