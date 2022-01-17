mod utils;

use druid::Widget;
use druid::widget::Flex;

use utils::CounterColumn;

pub struct ThreeColumnCounter;

impl ThreeColumnCounter {
    pub fn new() -> impl Widget<i64> {
        let mut flex = Flex::row();

        flex.add_child(utils::get_column_svg(CounterColumn::First));
        flex.add_child(utils::get_column_svg(CounterColumn::Second));
        flex.add_child(utils::get_column_svg(CounterColumn::Third));

        flex
    }
}