extern crate prusti_contracts;
use prusti_contracts::*;


#[pure]
fn id(i: i32)-> i32 {i}

#[ensures(forall(|i: i32| id(i) === id(i)))]
fn apply_id_rec_s(idx: i32) -> i32 {
    1
}



fn main() {

}
