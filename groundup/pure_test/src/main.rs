extern crate prusti_contracts;
use prusti_contracts::*;


#[pure]
#[ensures(result == 0)]
fn len(v: i32) -> i32 { 0 }


#[requires(len(vec) == 0)]
fn client(vec: i32) {}

fn main() {}
