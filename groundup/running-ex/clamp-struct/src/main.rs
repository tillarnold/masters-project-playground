extern crate prusti_contracts;
use prusti_contracts::*;

struct Bounds {
    min: i32,
    max: i32,
}

#[requires(bounds.min <= bounds.max)]
#[ensures(result<=bounds.max)]
#[ensures(result>=bounds.min)]
#[ensures((value<=bounds.max && value >= bounds.min) ==> result == value)]
fn clamp(value: i32, bounds: Bounds) -> i32 {
    if value < bounds.min {
        bounds.min
    } else if value > bounds.max {
        bounds.max
    } else {
        value
    }
}

#[ensures(result == 20)]
fn clamp_user(i: i32) -> i32 {
    clamp(i, Bounds{min: 10, max: 20})
}

fn foo(u: Bounds) {}

fn main() {}
