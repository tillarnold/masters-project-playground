extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}

// #[pure]
// fn f(a: i32, b: i32) -> i32 {
//     let q = if a > b {
//         if a < b {
//             if a == b {
//                 0
//             } else {
//                 44
//             }
//         } else if a < b {
//             if a == b {
//                 9
//             } else {
//                 12
//             }
//         }
//         else {
//             88
//         }
//     }
//     else {
//         7
//     };

//     if q == 9 {
//         100
//     }
//     else {
//         3
//     }
// }

#[pure]
fn f(a: i32, b: i32) -> i32 {
    if a > b {
        if a == b {
            0
        } else {
            44
        }
    } else {
        7
    }
}

#[requires(b)]
fn assert_true(b: bool) {}

fn main() {
    let val = f(100, 300);
    assert_true(!(val == 100));
}
