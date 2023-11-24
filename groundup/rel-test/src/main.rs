extern crate prusti_contracts;
use prusti_contracts::*;


#[ensures(rel(b, 0) == rel(a, 1) ==> rel(result, 0) == rel(result, 1))]
//#[ensures(result == old(a))]
fn id_2(a: i32, b: i32) -> i32 {
    a
}

fn main() {

}
