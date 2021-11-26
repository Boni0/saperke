use druid::Data;

#[derive(Clone, PartialEq, Data)]
pub enum GridShape {
    RectangleOrSquare,
    Unusual
}