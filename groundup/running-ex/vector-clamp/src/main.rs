extern crate prusti_contracts;
use prusti_contracts::*;


pub struct Vector {
    len: usize,
    contents: i32, // dummy variable encoding the contents of the vector
}

impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx < self.len)]
    fn get(self, idx: usize) -> i32 {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx < self.len)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: usize) -> (i32, Self) {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx < self.len)]
    #[ensures(self.len == result.len)]
    #[ensures(result.get(idx) == value)]
    #[ensures(forall(|i : usize| (i < self.len && i != idx) ==> result.get(i) == self.get(i)))]
    fn set(self, idx: usize, value: i32) -> Self {
        unimplemented!()
    }
}

struct Bounds {
    max: i32,
    min: i32,
}



#[requires(bounds.min <= bounds.max)]
#[ensures(result.0.len === vec.len)]
#[ensures(result.1 === bounds)]
#[ensures(forall(|ip: usize| (ip < vec.len) ==> result.0.get(ip) == bounds.clamp(vec.get(ip))))]
fn clamp_vector(bounds: Bounds, vec: Vector) -> (Vector, Bounds) {
    if vec.len == 0 {
        return (vec, bounds);
    }

    let len = vec.len;
    clamp_vector_rec(bounds, vec, len - 1)
}

#[requires(bounds.min <= bounds.max)]
#[requires(vec.len >= 1)]
#[requires(idx < vec.len)]
#[ensures(result.0.len === vec.len)]
#[ensures(result.1 === bounds)]
#[ensures(forall(|i: usize| ((i > idx) && (i < vec.len)) ==> result.0.get(i) == vec.get(i)))]
#[ensures(forall(|i: usize| ((i <= idx) && (i < vec.len)) ==> result.0.get(i) == bounds.clamp(vec.get(i))))]
fn clamp_vector_rec(
    bounds: Bounds,
    vec: Vector,
    idx: usize,
) -> (Vector, Bounds) {
    let (modified, bounds) = if idx >= 1 {
        clamp_vector_rec(bounds, vec, idx - 1)
    } else {
        (vec, bounds)
    };

    let (cur, vec) = modified.impure_get(idx);
    let (new, bounds) = bounds.impure_clamp(cur);
    let modified = vec.set(idx, new);
    (modified, bounds)
}

impl Bounds {
    #[pure]
    #[requires(self.min <= self.max)]
    #[ensures(result.0 === self.clamp(data))]
    #[ensures(result.1 === self)]
    fn impure_clamp(self, data: i32) -> (i32, Self) {
        if data < self.min {
            (self.min, self)
        } else if data > self.max {
            (self.max, self)
        } else {
            (data, self)
        }
    }

    #[pure]
    #[requires(self.min <= self.max)]
    fn clamp(self, data: i32) -> i32 {
        if data < self.min {
            self.min
        } else if data > self.max {
            self.max
        } else {
            data
        }
    }
}

#[requires(i >= 0)]
fn assert_geq_0(i: i32) {}

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
fn between(val: i32, min: i32, max: i32) -> bool {
    val <= max && val >= min
}

#[requires(forall(|i: usize| i< res.len ==> {let value = res.get(i); min <= value && value <= max }))]
fn final_assert(res: Vector, min: i32, max: i32) {}

#[requires(min <= max)]
pub fn client(data: Vector, min: i32, max: i32) {
    let t = Bounds { min, max };
    let (res, t2) = clamp_vector(t, data);

    final_assert(res, min, max);
}

/// This function shows that the second field in vector is required
/// if that field is removed this verifies implying that all vectors of the same length are the same
#[requires(data1.len == data2.len)]
#[requires(data1 !== data2)]
#[cfg(f)]
pub fn client2(data1: Vector, data2: Vector) {
    assert_true(false);
}

fn main() {}
