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
#[ensures(result.0.len === vec.len && result.1 === bounds)]
#[ensures(forall(|ip: usize| (ip < vec.len) ==> result.0.get(ip) == bounds.clamp(vec.get(ip))))]
fn clamp_vector(bounds: Bounds, vec: Vector) -> (Vector, Bounds) {
    if vec.len == 0 {
        return (vec, bounds);
    }

    let len = vec.len;
    clamp_vector_rec(bounds, vec, len - 1)
}

#[requires(bounds.min <= bounds.max)]
#[requires(idx < vec.len)]
#[ensures(result.0.len === vec.len && result.1 === bounds)]
#[ensures(forall(|i: usize| (i > idx  && i < vec.len) ==> result.0.get(i) == vec.get(i)))]
#[ensures(forall(|i: usize| (i <= idx && i < vec.len) ==> result.0.get(i) == bounds.clamp(vec.get(i))))]
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

    let (current_value, vec) = modified.impure_get(idx);
    let (new_value, bounds) = bounds.impure_clamp(current_value);
    (vec.set(idx, new_value), bounds)
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

#[requires(min <= max)]
#[ensures(forall(|i: usize| i < result.len ==> {let value = result.get(i); min <= value && value <= max }))]
pub fn client(vec: Vector, min: i32, max: i32) -> Vector {
    let bounds = Bounds { min, max };
    let (resulting_vec, _) = clamp_vector(bounds, vec);

    resulting_vec
}


fn main() {}
