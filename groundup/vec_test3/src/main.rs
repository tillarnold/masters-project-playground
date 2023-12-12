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




#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
//#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); (r === v)  }))]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {(((i > idx) | (i <= idx)) ==> result.get(i) === data.get(i)) }))]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); ((i > idx || i <= idx) ==> r === v) }))]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); (i > idx ==> r === v) && (i <= idx ==> r === v)  }))]
#[ensures(forall(|i: i32| (i >= 0 && i > idx && i < data.len)  ==> result.get(i) === data.get(i)))]
#[ensures(forall(|i: i32| (i >= 0 && i <= idx && i < data.len)  ==> result.get(i) === data.get(i)))]
fn apply_id_rec_s(data: Vector, idx: i32) -> Vector {
    data
}



#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {( result.get(i) === data.get(i))  }))] // should not be needed
// #[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {(((i > idx) | (i <= idx)) ==> result.get(i) === data.get(i)) }))]
// #[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); ((i > idx || i <= idx) ==> r === v) }))]
// #[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); (i > idx ==> r === v) && (i <= idx ==> r === v)  }))]
#[ensures(forall(|i: i32| (i >= 0 && i > idx && i < data.len)  ==> result.get(i) === data.get(i)))]
#[ensures(forall(|i: i32| (i >= 0 && i <= idx && i < data.len)  ==> result.get(i) === data.get(i)))]
fn apply_id_rec(data: Vector, idx: i32) -> Vector {
    if idx >= 1 {
        apply_id_rec(data, idx - 1)
    } else {
        data
    }
}


#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); let r = result.get(i); (r === v)  }))]
#[ensures(forall(|i: i32| (i >= 0 && i < data.len) ==> {let v = data.get(i); true  }))]
fn apply_id_rec_w(data: Vector, idx: i32) -> Vector {
    if idx >= 1 {
        apply_id_rec_w(data, idx - 1)
    } else {
        data
    }
}


// #[pure]
// #[trusted]
// fn id(v: i32) -> i32 {
//     0
// }


// #[ensures(forall(|i: i32| i === 3))]
// fn working() -> i32 {
//     0
// }



// #[ensures(forall(|i: i32| ( i < data.len) ==> {let v = data.get(i); true  }))]
// fn apply_id_working(data: Vector) -> Vector {
//    data
// }

// #[ensures(forall(|i: i32| (i < data.len) ==> { true  }))]
// fn apply_id_broken(data: Vector) -> Vector {
//    data
// }





// #[ensures(forall(|i: i32| ( i  <= result) ))]
// fn x(val: i32, len: i32) -> i32{
//     42
// }






// #[ensures(forall(|i: i32| i === result))]
// fn broken() -> i32 {
//     0
// }

// #[requires(i >= 0)]
// fn assert_geq_0(i: i32) {}

// #[requires(b)]
// fn assert_true(b: bool) {}

// #[requires(a == b)]
// fn assert_eq(a: i32, b: i32) {}

// #[requires(a === b)]
// fn assert_eq_snap(a: Vector, b: Vector) {}

fn main() {

}
