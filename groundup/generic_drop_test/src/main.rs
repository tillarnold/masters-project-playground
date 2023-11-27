extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector<T> {
    len: i32,
    contents: i32, // dummy variable encoding the contents of the vector
    el: T,
}

impl<T> Vector<T> {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    fn get(self, idx: i32) -> T {
        unimplemented!()
    }
}


#[requires(vec.len == 10)]
fn vector_client(vec: Vector<i32>) {
    let res = vec.get(5);
}

fn main() {

}
