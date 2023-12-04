extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector/*<T>*/ {
    len: usize,
    contents: i32, // dummy variable encoding the contents of the vector
    //el: T,
}

impl/*<T>*/ Vector/*<T>*/ {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    fn get(self, idx: usize) -> i32/*T*/ {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(result.0 === self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: usize) -> (i32/*T*/, Self) {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(self.len == result.len)]
    #[ensures(result.get(idx) === value)]
    #[ensures(forall(|i : usize| ((i >= 0)  & (i < self.len) & (i != idx)) ==> result.get(i) == self.get(i)))]
    fn set(self, idx: usize, value: i32/*T*/) -> Self {
        unimplemented!()
    }
}

#[requires(b)]
#[pure]
fn assert_true(b: bool) {}

#[requires(a == b)]
#[pure]
fn assert_eq(a: i32, b: i32) {}


#[requires(a === b)]
#[pure]
#[trusted]
fn assert_eq_snap<T>(a: T, b: T) {}

#[requires(vec.len == 10)]
fn vector_client(vec: Vector/*<i32>*/) {
    let vec = vec.set(5, 42);
    let res = vec.get(5);
    assert_eq(res, 42);
}


fn main() {

}
