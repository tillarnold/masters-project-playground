extern crate prusti_contracts;
use prusti_contracts::*;

struct Bounds {
    min: i32,
    max: i32,
}

#[pure]
#[ensures(result === bounds)]
fn id(mut bounds: Bounds) -> Bounds {
    return bounds;
}




fn main() {
    id(Bounds{min: 10, max: 10});
}

