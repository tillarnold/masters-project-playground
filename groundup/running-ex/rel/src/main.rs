extern crate prusti_contracts;
use prusti_contracts::*;

struct Bounds {
    min: i32,
    max: i32,
}

enum Result { Ok(i32), Err }

#[pure]
#[ensures(match result {
    Result::Err => true,
    Result::Ok(value) => value <= bounds.max &&  bounds.min <= value
})]
#[ensures((bounds.min <= value && value <= bounds.max) ==> (
    match result {
        Result::Err => false,
        Result::Ok(rvalue) => value == rvalue
    }
))]
#[ensures( rel0(&bounds) === rel1(&bounds) ==> match (rel0(&result), rel1(&result)) {
    (Result::Err,Result::Err) => true,
    (Result::Ok(_),Result::Ok(_)) => true,
    _ => false,
})]
fn clamp(value: i32, bounds: Bounds) -> Result {
    if bounds.min > bounds.max { Result::Err }
    else if value < bounds.min { Result::Ok(bounds.min) }
    else if value > bounds.max { Result::Ok(bounds.max) }
    else { Result::Ok(value) }
}

fn main() {}
