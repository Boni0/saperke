use super::config::SizeUnit;
use druid::{Data, Lens};

#[derive(Clone, PartialEq, Lens, Data)]
pub struct GridSize {
    pub width: SizeUnit,
    pub height: SizeUnit,
}
