extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: i32,
}

impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    fn get(self, idx: i32) -> i32 {
        unimplemented!()
    }


}


#[requires(data1.len == 1)]
#[requires(data2.len == 1)]
#[requires(data1.get(0) != data2.get(0))]
#[ensures(false)]
pub fn client(data1: Vector, data2: Vector) {
}



fn main() {

}