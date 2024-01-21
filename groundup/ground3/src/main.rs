extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: usize,
    contents: i32, // dummy variable encoding the contents of the vector
}

impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    fn get(self, idx: usize) -> i32 {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: usize) -> (i32, Self) {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
    #[ensures(self.len == result.len)]
    #[ensures(result.get(idx) == value)]
    #[ensures(forall(|i : usize| (i >= 0  && i < self.len && !(i == idx)) ==> result.get(i) == self.get(i)))]
    fn set(self, idx: usize, value: i32) -> Self {
        unimplemented!()
    }
}

struct Bounds {
    upper: i32,
    lower: i32,
}

struct ClampTransform {
    bounds: Bounds,
}

// #[requires(data.len >= 0)]
// #[requires(transform.bounds.lower < transform.bounds.upper)]
// #[ensures(result.0.len === data.len)]
// #[ensures(result.1 === transform)]
// #[ensures(forall(|ip: usize| (0<= ip) & (ip < data.len) ==> result.0.get(ip) == transform.do_transform(data.get(ip))))]
// fn apply_row_by_row(transform: ClampTransform, data: Vector/*<i32>*/) -> (Vector/*<i32>*/, ClampTransform) {
//     if data.len <= 0 {
//         return (data, transform);
//     }

//     let l = data.len;
//     assert_true( l >= 1);
//     assert_true( l - 1 >= 0);
//     assert_true( l - 1 < l);
//     apply_row_by_row_rec(transform, data, l - 1)
// }

// #[requires(transform.bounds.lower < transform.bounds.upper)]
// #[requires(idx >= 0)]
// #[requires(data.len >= 1)]
// #[requires(idx < data.len)]
// #[ensures(result.0.len === data.len)]
// #[ensures(result.1 === transform)]
// #[ensures(forall(|i: usize| ((i >= 0) && (i > idx) && (i < data.len)) ==> result.0.get(i) == data.get(i)))]
// #[ensures(forall(|i: usize| ((i >= 0) && (i <= idx) && (i < data.len)) ==> result.0.get(i) == transform.do_transform(data.get(i))))]
// fn apply_row_by_row_rec(transform: ClampTransform, data: Vector/*<i32>*/, idx: usize) -> (Vector/*<i32>*/, ClampTransform) {
//     let (modified, transform) = if idx >= 1 {
//         apply_row_by_row_rec(transform, data, idx - 1)
//     } else {
//         (data, transform)
//     };

//     let (cur, data) = modified.impure_get(idx);
//     let (new, transform) = transform.do_transform_impure(cur);
//     let modified = data.set(idx, new);
//     (modified, transform)
// }


#[requires(idx < 1000)]
#[requires(data.len == 2000)]
#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
#[ensures(forall(|i: usize| (i >= 0 && i < data.len) ==> ( result.get(i) === data.get(i))  ))] 
#[ensures(forall(|i: usize| (i >= 0 && i > idx && i < data.len)  ==> result.get(i) === data.get(i)))]
#[ensures(forall(|i: usize| (i >= 0 && i <= idx && i < data.len)  ==> result.get(i) === data.get(i)))]
fn apply_id_rec(data: Vector, idx: usize) -> Vector {
    if idx + 1 < 10 {
        apply_id_rec(data, idx + 1)
    } else {
        data
    }
}



// #[requires(idx >= 0)]
// #[requires(data.len >= 1)]
// #[requires(idx < data.len)]
// #[ensures(result.len === data.len)]
// #[ensures(forall(|i: usize| ((i >= 0) && (i < data.len)) ==> result.get(i) == (data.get(i))))]
// // #[ensures(forall(|i: usize| ((i >= 0) && (i > idx) && (i < data.len)) ==> result.get(i) == data.get(i)))]
// // #[ensures(forall(|i: usize| ((i >= 0) && (i <= idx) && (i < data.len)) ==> result.get(i) == (data.get(i))))]
// fn id_rec(data: Vector, idx: usize) -> Vector {
//     let (modified) = if idx >= 1 {
//         id_rec(data, idx - 1)
//     } else {
//         (data)
//     };

//     let (cur, data) = modified.impure_get(idx);
//     let modified = data.set(idx, cur);
//     (modified)
// }



#[requires(i >= 0)]
fn assert_geq_0(i: i32) {}

#[pure]
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[pure]
fn min(a: i32, b: i32) -> i32 {
    if a > b {
        b
    } else {
        a
    }
}

impl ClampTransform {
    #[ensures(result.bounds === bounds)]
    fn make_clamp(bounds: Bounds) -> Self {
        Self { bounds }
    }

    #[pure]
    #[requires(self.bounds.lower < self.bounds.upper)]
    fn do_transform(self, data: i32) -> i32 {
        // implemented like this due to limitation in pure controll flow
        max(self.bounds.lower, min(self.bounds.upper, data))
    }

    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(result.0 == self.do_transform(data))]
    #[ensures(result.1 === self)]
    fn do_transform_impure(self, data: i32) -> (i32, Self) {
        if data < self.bounds.lower {
            (self.bounds.lower, self)
        } else if data > self.bounds.upper {
            (self.bounds.upper, self)
        } else {
            (data, self)
        }
    }
}

#[requires(b)]
fn assert_true(b: bool) {}

#[requires(a == b)]
fn assert_eq(a: i32, b: i32) {}


#[requires(a === b)]
fn assert_eq_snap(a: Vector, b: Vector) {}

#[requires(vec.len == 10)]
fn vector_client(vec: Vector) {
    let vec = vec.set(5, 42);
    let res = vec.get(5);
    assert_eq(res, 42);
}

#[pure]
fn between(val: i32, lower: i32, upper: i32) -> bool {
    val <= upper && val >= lower
}

#[requires(forall(|i: usize| (0<= i && i< res.len) ==> between(res.get(i), lower, upper)))]
fn final_assert(res: Vector, lower: i32, upper: i32) {}

// #[requires(data.len >= 10)]
// #[requires(lower < upper)]
// pub fn client(data: Vector, lower: i32, upper: i32) {
//     let t = ClampTransform::make_clamp(Bounds { lower, upper });
//     let (res, t2) = apply_row_by_row(t, data);

//     final_assert(res, lower, upper);
//     //prusti_assert!(forall(|i: i32| (0<= i && i< res.len) ==> res.get(i) <= 200 && res.get(i) >= 100))
// }


/// This function shows that the second field in vector is required
/// if that field is removed this verifies implying that all vectors of the same length are the same
#[requires(data1.len == data2.len)]
#[requires(data1 !== data2)]
#[cfg(f)]
pub fn client2(data1: Vector, data2: Vector) {
    assert_true(false);
}



fn main() {

}
