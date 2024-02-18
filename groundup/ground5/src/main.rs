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
#[ensures(result.1 === transform)]
#[ensures((transform.bounds.min <= transform.bounds.max || data.len == 0) <==>
    matches!(result.0, FallibleVec::Ok(_)))]
#[ensures(matches!(result.0, FallibleVec::Ok(_)) ==>
    result.0.unwrap_vec().len === data.len)]
#[ensures(matches!(result.0, FallibleVec::Ok(_)) ==>
    forall(|ip: usize|(ip < data.len) ==>
    result.0.unwrap_vec().get(ip) == transform.do_transform(data.get(ip))))]
#[ensures((rel0(&transform) === rel1(&transform) && rel0(&data.len) === rel1(&data.len)) ==> match (rel0(&result.0), rel1(&result.0)) {
    (FallibleVec::Err,FallibleVec::Err) => true,
    (FallibleVec::Ok(_),FallibleVec::Ok(_)) => true,
    _ => false,
})]
fn apply_row_by_row(transform: ClampTransform, data: Vector)
    -> (FallibleVec, ClampTransform) {
    if data.len <= 0 { return (FallibleVec::Ok(data), transform); }
    let l = data.len;
    apply_row_by_row_rec(transform, data, l - 1)
}

#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.1 === transform)]
#[ensures(transform.bounds.min <= transform.bounds.max <==>
    matches!(result.0, FallibleVec::Ok(_)))]
#[ensures(matches!(result.0, FallibleVec::Ok(_)) ==>
    result.0.unwrap_vec().len === data.len)]
#[ensures(matches!(result.0, FallibleVec::Ok(_)) ==>
    forall(|i: usize| (i > idx && i < data.len)  ==>
    result.0.unwrap_vec().get(i) == data.get(i)))]
#[ensures(matches!(result.0, FallibleVec::Ok(_)) ==>
    forall(|i: usize| (i <= idx && i < data.len) ==>
    result.0.unwrap_vec().get(i) == transform.do_transform(data.get(i))))]
#[ensures((rel0(idx) == rel1(idx) && rel0(&transform) === rel1(&transform) ) ==> match (rel0(&result.0), rel1(&result.0)) {
    (FallibleVec::Err,FallibleVec::Err) => true,
    (FallibleVec::Ok(_),FallibleVec::Ok(_)) => true,
    _ => false,
})]
fn apply_row_by_row_rec(
    mut transform: ClampTransform,
    mut data: Vector,
    idx: usize,
) -> (FallibleVec, ClampTransform) {
    if idx >= 1 {
        (data, transform) = match apply_row_by_row_rec(transform, data, idx - 1) {
            (FallibleVec::Ok(vec), transform) => (vec, transform),
            (FallibleVec::Err, transform) => return (FallibleVec::Err, transform),
        };
    }

    let (cur, data) = data.impure_get(idx);
    let (clamped, transform) = transform.do_transform_impure(cur);

    if let FallibleI32::Ok(clamped) = clamped {
        let data = data.set(idx, clamped);
        return (FallibleVec::Ok(data), transform);
    }

    (FallibleVec::Err, transform)
}

enum FallibleVec {
    Ok(Vector),
    Err,
}

enum FallibleI32 {
    Ok(i32),
    Err,
}

#[trusted]
#[pure]
#[requires(false)]
fn unreachable_i32() -> i32 {
    unreachable!()
}

#[trusted]
#[pure]
#[requires(false)]
fn unreachable_vec() -> Vector {
    unreachable!()
}

impl FallibleI32 {
    #[pure]
    #[requires(matches!(self, FallibleI32::Ok(_)))]
    fn unwrap_i32(self) -> i32 {
        match self {
            FallibleI32::Ok(val) => val,
            FallibleI32::Err => unreachable_i32(),
        }
    }
}

impl FallibleVec {
    #[pure]
    #[requires(matches!(self, FallibleVec::Ok(_)))]
    fn unwrap_vec(self) -> Vector {
        match self {
            FallibleVec::Ok(val) => val,
            FallibleVec::Err => unreachable_vec(),
        }
    }
}

impl ClampTransform {
    #[ensures(result.bounds === bounds)]
    fn make_clamp(bounds: Bounds) -> Self {
        Self { bounds }
    }

    #[pure]
    #[requires(self.bounds.min <= self.bounds.max)]
    fn do_transform(self, data: i32) -> i32 {
        if data < self.bounds.min {
            (self.bounds.min)
        } else if data > self.bounds.max {
            (self.bounds.max)
        } else {
            (data)
        }
    }

    #[ensures(self.bounds.min <= self.bounds.max <==> matches!(result.0, FallibleI32::Ok(_)))]
    #[ensures(self.bounds.min <= self.bounds.max ==> result.0.unwrap_i32() === self.do_transform(data))]
    #[ensures(result.1 === self)]
    #[ensures( rel0(&self.bounds) === rel1(&self.bounds) ==> match (rel0(&result.0), rel1(&result.0)) {
        (FallibleI32::Err,FallibleI32::Err) => true,
        (FallibleI32::Ok(_),FallibleI32::Ok(_)) => true,
        _ => false,
    })]
    fn do_transform_impure(self, data: i32) -> (FallibleI32, Self) {
        if self.bounds.min > self.bounds.max {
            (FallibleI32::Err, self)
        } else if data < self.bounds.min {
            (FallibleI32::Ok(self.bounds.min), self)
        } else if data > self.bounds.max {
            (FallibleI32::Ok(self.bounds.max), self)
        } else {
            (FallibleI32::Ok(data), self)
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

// #[requires(min <= max)]
pub fn client(data: Vector, min: i32, max: i32) {
    let t = ClampTransform::make_clamp(Bounds { min, max });
    let (res, t2) = apply_row_by_row(t, data);
    match res {
        FallibleVec::Ok(vec) => {
            final_assert(vec, min, max);
        }
        _ => {}
    }
}

fn main() {}
