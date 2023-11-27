extern crate prusti_contracts;
use prusti_contracts::*;


#[requires(b)]
fn assert_true(b: bool) {}



fn main() {
    assert_true( 1 + 1 > 1);
}
