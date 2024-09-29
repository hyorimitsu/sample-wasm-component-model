#[allow(warnings)]
mod bindings;

use bindings::exports::component::calculator::calculate::{Guest, Op};
use bindings::component::{adder::add::add, subtractor::subtract::subtract};

struct Component;

impl Guest for Component {
    fn calculate(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
            Op::Sub => subtract(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
