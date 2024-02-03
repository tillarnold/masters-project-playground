extern crate prusti_contracts;
use prusti_contracts::*;


#[pure]
fn returns_int() -> i32 {
    if true {
        return 0;
    }

    return 0;
}

#[requires(returns_int() === 0)]
fn client(i: i32) {}

fn main() {}
