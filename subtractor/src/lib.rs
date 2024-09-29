#[allow(warnings)]
mod bindings;

use bindings::exports::component::subtractor::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(x: u32, y: u32) -> u32 {
        x - y
    }
}

bindings::export!(Component with_types_in bindings);
