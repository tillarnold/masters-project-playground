extern crate prusti_contracts;
use prusti_contracts::*;

#[requires(b)]
fn assert(b: bool) {}

enum IntOption {
    None,
    Some,
}

#[ensures(result == (s === IntOption::None))]
fn is_none(s: IntOption) -> bool {
    matches!(s, IntOption::None)
}

fn main() {
    is_none(IntOption::None);
}
