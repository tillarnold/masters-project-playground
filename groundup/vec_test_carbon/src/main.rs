extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: i32,
    contents: i32, // dummy variable encoding the contents of the vector
}

impl Vector {
    #[pure]
    #[trusted]
    fn get(self, idx: i32) -> i32 {
        unimplemented!()
    }
}



#[ensures(forall(|i: i32|  result.get(i) === data.get(i)))]
fn apply_id_rec_s(data: Vector, idx: i32) -> Vector {
    data
}



fn main() {

}
