#![feature(core_intrinsics, rustc_attrs)]

extern crate prusti_contracts;
use prusti_contracts::*;



#[requires(b)]
fn assert(b: bool) {}

enum IntOption {
    Some(i32),
    None,
}

impl IntOption {
    #[rustc_mir(borrowck_graphviz_postflow="log/analysis/fn_name/fn_name.dot")]
    #[ensures(self === IntOption::None ==> result == default)]
    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            IntOption::Some(v) => v,
            IntOption::None => default,
        }
    }

}
fn main() {
    let v = IntOption::None.unwrap_or(22);
    assert(v == 22);
}

