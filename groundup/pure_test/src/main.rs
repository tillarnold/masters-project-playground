extern crate prusti_contracts;
use prusti_contracts::*;


#[requires(true)]
#[ensures(true)]
fn len() -> i32 { 0 }


fn main() {
    len();
}
