extern crate prusti_contracts;
use prusti_contracts::*;

#[requires(b)]
fn assert_true(b: bool) {}

#[requires(a == b)]
fn assert_eq(a: i32, b: i32) {}

#[requires(x >= 0)]
fn f(x: i32) {
    if x < 1 {
        assert_eq(x, 0)
    }
}

#[requires(a >= 0)]
fn g(a: i32) {
    if a < 1 {
        assert_true(a == 0);
    }
}

fn main() {}
