extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: i32,
    contents: i32, // dummy variable encoding the contents of the vector
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





#[requires(idx < 1000)]
#[requires(data.len == 2000)]
#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
// #[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> ( result.get(i) === data.get(i))  ))] 
#[ensures(forall(|i: i32| (i >= 0 && i > idx && i < data.len)  ==> result.get(i) === data.get(i)))]
#[ensures(forall(|i: i32| (i >= 0 && i <= idx && i < data.len)  ==> result.get(i) === data.get(i)))]
fn apply_id_rec(data: Vector, idx: i32) -> Vector {
    if idx + 1 < 10 {
        apply_id_rec(data, idx + 1)
    } else {
        data
    }
}


// #[requires(a < 10000)]
// fn add(a: i32) -> i32 {
//     a + 1
// }



fn main () {}