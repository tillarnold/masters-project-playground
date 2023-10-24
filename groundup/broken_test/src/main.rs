extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}




/// folding
struct Test {
    x: i32
}


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

