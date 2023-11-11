#![feature(core_intrinsics, rustc_attrs)]

extern crate prusti_contracts;
use prusti_contracts::*;

#[requires(b)]
fn assert(b: bool) {}

enum IntOption {
    Some(i32),
    None,
}

#[rustc_mir(borrowck_graphviz_postflow = "log/analysis/fn_name/fn_name.dot")]
#[ensures(s === IntOption::None ==> result == default)]
fn unwrap_or(s: IntOption, default: i32) -> i32 {
    match s {
        IntOption::Some(v) => v,
        IntOption::None => default,
    }
}

fn main() {
    let v = unwrap_or(IntOption::None, 22);
    assert(v == 22);
}
