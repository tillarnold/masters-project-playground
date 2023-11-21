
extern crate prusti_contracts;
use prusti_contracts::*;



#[pure]
fn is_twelve(value: &i32) -> bool {
    value == &12
}

#[requires(b)]
fn assert_true(b: bool) {

}


fn main() {
    // let t = is_twelve(&12);
    // let f = is_twelve(&1);
    // assert_true(t);
    // assert_true(!f);
}

