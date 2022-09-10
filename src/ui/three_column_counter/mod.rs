mod utils;

use druid::widget::Flex;
use druid::Widget;

use utils::{CounterColumn, CounterColumnPainter};

pub struct ThreeColumnCounter;

impl ThreeColumnCounter {
    pub fn new() -> impl Widget<i64> {
        let painter = CounterColumnPainter::create();
        let mut flex = Flex::row();

        flex.add_child(CounterColumn::new(CounterColumn::First, &painter));
        flex.add_child(CounterColumn::new(CounterColumn::Second, &painter));
        flex.add_child(CounterColumn::new(CounterColumn::Third, &painter));

        flex
    }
}
