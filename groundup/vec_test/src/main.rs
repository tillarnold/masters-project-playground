extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}

struct Vector {
    dummy: i32,
}

impl Vector {

    #[pure]
    #[trusted]
    #[ensures(result >= 0)]
    fn len(&self) -> i32 {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len())]
    fn get_pure(&self, idx: i32) -> i32 {
        unimplemented!()
    }

}

#[pure]
fn f(v: Vector) -> i32{
    v.len()
}


fn main() {

}

