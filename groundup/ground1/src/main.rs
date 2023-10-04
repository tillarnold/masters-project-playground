extern crate prusti_contracts;
use prusti_contracts::*;

#[derive(Clone)]
struct Vector {}

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
    fn get(&self, idx: i32) -> i32 {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len())]
    #[ensures(self.len() == result.len())]
    #[ensures(result.get(idx) == value)]
    #[ensures(forall(|i : i32| (i >= 0  && i < self.len() && i != idx) ==> result.get(i) == self.get(i)))]
    fn set(self, idx: i32, value: i32) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Copy)]
struct Bounds {
    upper: i32,
    lower: i32,
}

impl Bounds {
    #[pure]
    fn new(lower: i32, upper: i32) -> Self {
        Bounds { upper, lower }
    }
}

#[derive(Clone, Copy)]
struct ClampTransform {
    bounds: Bounds,
}


#[ensures(transform === old(transform))]
#[ensures(result.len() === old(data).len())]
#[ensures(forall(|ip: i32| (0<= ip && ip < old(snap(&data)).len() )  ==> result.get(ip) == transform.do_transform((old(snap(&data))).get(ip)) ))]
fn apply_row_by_row(transform: ClampTransform, data: Vector) -> Vector {
    apply_row_by_row_rec(transform, data, 0)
}


#[requires(i >= 0)]
#[ensures(transform === old(transform))]
#[ensures(result.len() === old(data).len())]
#[ensures(forall(|ip: i32| (i<= ip && ip < old(snap(&data)).len())  ==> result.get(ip) == transform.do_transform((old(snap(&data))).get(ip)) ))]
fn apply_row_by_row_rec(transform: ClampTransform, data: Vector, i: i32) -> Vector {
    if i >= data.len() {
        return data;
    } else {
        let cur = data.get(i);
        let new = transform.do_transform(cur);
        let data = data.set(i, new);

        apply_row_by_row_rec(transform, data, i + 1)
    }
}

impl ClampTransform {
    #[pure]
    fn make_clamp(bounds: Bounds) -> Self {
        Self { bounds }
    }

    #[pure]
    fn do_transform(&self, data: i32) -> i32 {
        if data < self.bounds.lower {
            self.bounds.lower
        } else if data > self.bounds.upper {
            self.bounds.upper
        } else {
            data
        }
    }
}

#[requires(vec.len() == 10)]
fn vector_client(vec: Vector) {
    let vec = vec.set(5, 42);
    let res = vec.get(5);
    assert!(res == 42);
}

fn client(data: Vector) {
    let t = ClampTransform::make_clamp(Bounds::new(100, 200));
    let res = apply_row_by_row(t, data);
    prusti_assert!(forall(|i: i32| (0<= i && i< res.len()) ==> res.get(i) <= 200 && res.get(i) >= 100))
}

fn main() {}
