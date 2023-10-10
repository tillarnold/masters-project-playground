extern crate prusti_contracts;
use prusti_contracts::*;


fn return_a_bool() ->  bool {
    true
}

#[ensures(result >= 0)]
fn len() -> i32 {
    -1
}

#[requires(i >= 0)]
#[ensures(result >= i)]
fn len2(i: i32) -> i32 {
    i
}

#[ensures(result == i)]
fn id(i: i32) -> i32 {
    i
}


#[requires(i >= 0)]
fn assert_ge_0(i: i32) {

}

fn main() {
    let x = len();
    let y = len2(x);
    let z = id(y);
    assert_ge_0(z);
}
