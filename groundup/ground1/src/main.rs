extern crate prusti_contracts;
use prusti_contracts::*;

struct Vector {}

impl Vector {
    #[pure]
    #[trusted]
    fn len(self) -> i32 {
        todo!()
    }

    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len())]
    fn get(self, idx: i32) -> i32 {
        todo!()
    }

    #[trusted]
    fn set(self, idx: i32, value: i32) -> Self {
        todo!()
    }
}


#[requires(vec.len() == 10)]
fn vector_client(vec: Vector) {
    vec.get(3);

}

fn main() {
}
