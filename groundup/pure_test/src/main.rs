extern crate prusti_contracts;
use prusti_contracts::*;

#[ensures(result >= 0)]
fn len() -> i32 {
    0
}


fn main() {
    let x = len();
    prusti_assert!(x == 0);
}
