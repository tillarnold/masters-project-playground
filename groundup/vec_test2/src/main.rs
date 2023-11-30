extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: usize,
    contents: i32, // dummy variable encoding the contents of the vector
}

impl Vector {
    #[pure]
    #[trusted]
    fn get(self, idx: usize) -> i32/*T*/ {
        unimplemented!()
    }


    #[trusted]
    #[ensures(result.get(idx) === value)]
    fn set(self, idx: usize, value: i32/*T*/) -> Self {
        unimplemented!()
    }
}

#[requires(a == b)]
#[pure]
fn assert_eq(a: i32, b: i32) {}


#[requires(vec.len == 10)]
fn vector_client(vec: Vector) {
    let vec = vec.set(5, 42);
    let res = vec.get(5);
    assert_eq(res, 42);
}

fn main() {

}
