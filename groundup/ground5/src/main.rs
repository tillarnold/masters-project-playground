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

struct ClampTransform {
    bounds: Bounds,
}

#[requires(transform.bounds.min <= transform.bounds.max)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|ip: usize| (ip < data.len) ==> result.0.get(ip) == transform.do_transform(data.get(ip)).unwrap()))]
fn apply_row_by_row(transform: ClampTransform, data: Vector) -> (Vector, ClampTransform) {
    if data.len <= 0 {
        return (data, transform);
    }

    let l = data.len;
    apply_row_by_row_rec(transform, data, l - 1)
}

#[requires(transform.bounds.min <= transform.bounds.max)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|i: usize| ((i > idx) && (i < data.len)) ==> result.0.get(i) == data.get(i)))]
#[ensures(forall(|i: usize| ((i <= idx) && (i < data.len)) ==> result.0.get(i) == transform.do_transform(data.get(i)).unwrap()))]
fn apply_row_by_row_rec(
    transform: ClampTransform,
    data: Vector, 
    idx: usize,
) -> (Vector, ClampTransform) {
    let (modified, transform) = if idx >= 1 {
        apply_row_by_row_rec(transform, data, idx - 1)
    } else {
        (data, transform)
    };

    let (cur, data) = modified.impure_get(idx);
    let (new, transform) = transform.do_transform_impure(cur);
    let modified = data.set(idx, new);
    (modified, transform)
}



enum FallibleI32 {
    Ok(i32),
    Err
}


#[trusted]
#[pure]
#[requires(false)]
fn assert_false() -> i32{
    unreachable!()
} 

impl FallibleI32 {
    #[pure]
    #[requires(matches!(self, FallibleI32::Ok(_)))]
    fn unwrap(self) -> i32 {
        match self{
            FallibleI32::Ok(val) => val,
            FallibleI32::Err => assert_false()
        }
    }
}

impl ClampTransform {
    #[ensures(result.bounds === bounds)]
    fn make_clamp(bounds: Bounds) -> Self {
        Self { bounds }
    }


    #[pure]
    fn do_transform(self, data: i32) -> FallibleI32 {
        if self.bounds.min > self.bounds.max {
            FallibleI32::Err
        }
        else if data < self.bounds.min {
            FallibleI32::Ok(self.bounds.min)
        } else if data > self.bounds.max {
            FallibleI32::Ok(self.bounds.max)
        } else {
            FallibleI32::Ok(data)
        }
    }

    #[requires(self.bounds.min <= self.bounds.max)]
    #[ensures(result.0 == self.do_transform(data).unwrap())]
    #[ensures(result.1 === self)]
    fn do_transform_impure(self, data: i32) -> (i32, Self) {
        if data < self.bounds.min {
            (self.bounds.min, self)
        } else if data > self.bounds.max {
            (self.bounds.max, self)
        } else {
            (data, self)
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

#[requires(forall(|i: usize| i< res.len ==> {let value = res.get(i); min <= value && value <= max }))]
fn final_assert(res: Vector, min: i32, max: i32) {}

#[requires(min <= max)]
pub fn client(data: Vector, min: i32, max: i32) {
    let t = ClampTransform::make_clamp(Bounds { min, max });
    let (res, t2) = apply_row_by_row(t, data);

    final_assert(res, min, max);
}


fn main() {}
