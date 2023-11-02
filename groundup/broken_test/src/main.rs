#![feature(core_intrinsics, rustc_attrs)]

extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}




/// folding
struct Test {
    x: i32
}


#[rustc_mir(borrowck_graphviz_postflow="log/analysis/fn_name/fn_name.dot")]
fn take_test(test: Test) -> Test {
    if test.x > 20 {
        return test;
    }

    return Test {
        x: 100
    }
}



// // Making struct in pure function
// struct Foo {
//     a: i32, 
//     b: i32,
// }

// #[pure]
// fn make_foo() {
//     let f = Foo {a: 100, b: 100};
// }

fn main() {
    
}

