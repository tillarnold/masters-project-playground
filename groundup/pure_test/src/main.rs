extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}

#[pure]
#[ensures(result >= 0)]
fn f1() -> i32 {
    0
}

#[pure]
#[ensures(result == i)]
fn id(i: i32) -> i32 {
    i
}

#[pure]
#[ensures(result >= 0)]
fn f2(b1: bool, b2: bool) -> i32 {
    id(f1())
}

#[requires(i >= 0)]
#[ensures(result >= i)]
#[pure]
fn len2(i: i32) -> i32 {
    i
}

#[requires(i >= 0)]
fn assert_ge_0(i: i32) {}

fn main() {
    let x = f1();
    let y = len2(x);
    let z = id(y);
    assert_ge_0(z);
}

