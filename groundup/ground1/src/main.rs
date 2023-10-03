extern crate prusti_contracts;
use prusti_contracts::*;

struct Vector {}

impl Vector {
    #[pure]
    #[trusted]
    fn len() -> i32 {
        todo!()
    }

    fn get(&self, idx: i32) -> i32 {
        todo!()
    }

    fn set(self, idx: i32, value: i32) -> Self {
        todo!()
    }
}

fn main() {
}
