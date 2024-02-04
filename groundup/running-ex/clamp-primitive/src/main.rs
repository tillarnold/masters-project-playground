extern crate prusti_contracts;
use prusti_contracts::*;



#[pure]
#[requires(min <= max)]
#[ensures(result <= max)]
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    }
    else if value > max {
        max
    }
    else {
        value
    }
}

#[requires(clamp(i, 10, 20) == 20)]
fn clamp_user(i: i32) {

}

fn main() {

}
