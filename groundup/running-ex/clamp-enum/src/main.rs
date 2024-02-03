extern crate prusti_contracts;
use prusti_contracts::*;

struct Bounds {
    min: i32,
    max: i32,
}

enum Result {
    Ok(i32),
    Err,
}

#[pure]
#[ensures(match result {
    Result::Err => true,
    Result::Ok(value) => value <= bounds.max
})]
fn clamp(value: i32, bounds: Bounds) -> Result {
    if bounds.min > bounds.max {
        return Result::Err;
    }

    if value < bounds.min {
        Result::Ok(bounds.min)
    } else if value > bounds.max {
        Result::Ok(bounds.max)
    } else {
        Result::Ok(value)
    }
}

#[requires(clamp(i, Bounds{min: 10, max: 20}) === Result::Ok(20))]
fn clamp_user(i: i32) {}

fn main() {}
