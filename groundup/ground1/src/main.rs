extern crate prusti_contracts;
use prusti_contracts::*;

struct Vector {
    len: i32,
}

impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    fn get(self, idx: i32) -> i32 {
        unimplemented!()
    }

    // #[trusted]
    // #[requires(idx >= 0)]
    // #[requires(idx < self.len)]
    // #[ensures(result.0 == self.get(idx))]
    // #[ensures(result.1.veq(self))]
    // fn impure_get(self, idx: i32) -> (i32, Self) {
    //     unimplemented!()
    // }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: i32) -> (i32, Self) {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[ensures(self.len == result.len)]
    #[ensures(result.get(idx) == value)]
    #[ensures(forall(|i : i32| (i >= 0  && i < self.len && !(i == idx)) ==> result.get(i) == self.get(i)))]
    fn set(self, idx: i32, value: i32) -> Self {
        unimplemented!()
    }

    // #[trusted]
    // #[pure]
    // #[ensures(result ==> (self.len == other.len))]
    // #[ensures(result ==> forall(|i : i32| (i >= 0  && i < self.len) ==> other.get(i) == self.get(i)))]
    // fn veq(self, other: Vector) -> bool {   
    //     unimplemented!()
    // }

}



#[derive(Clone, Copy)]
struct Bounds {
    upper: i32,
    lower: i32,
}


#[derive(Clone, Copy)]
struct ClampTransform {
    bounds: Bounds,
}


#[requires(transform.bounds.lower < transform.bounds.upper)]
#[ensures(transform === (transform))]
#[ensures(result.len === (data).len)]
#[ensures(forall(|ip: i32| (0<= ip && ip < (data).len)  ==> result.get(ip) == transform.do_transform(data.get(ip)) ))]
fn apply_row_by_row(transform: ClampTransform, data: Vector) -> Vector {
    apply_row_by_row_rec(transform, data, 0)
}


#[requires(transform.bounds.lower < transform.bounds.upper)]
#[requires(i >= 0)]
#[ensures(transform === (transform))]
#[ensures(result.len === (data).len)]
#[ensures(forall(|ip: i32| (i<= ip && ip < ((data)).len)  ==> result.get(ip) == transform.do_transform(data.get(ip)) ))]
fn apply_row_by_row_rec(transform: ClampTransform, data: Vector, i: i32) -> Vector {
    if i >= data.len {
        return data;
    } else {
        let (cur, data) = data.impure_get(i);
        let new = transform.do_transform(cur);
        let data = data.set(i, new);

        assert_geq_0(i);
        let next = i + 1;
        assert_geq_0(next);
        apply_row_by_row_rec(transform, data, next)
    }
}

#[requires(i >= 0)]
fn assert_geq_0(i : i32) {

}

#[pure]
#[ensures( a > b ==> result == a)]
#[ensures( b > a ==> result == b)]
#[ensures( a == b ==> result == b)]
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

#[pure]
#[ensures( a > b ==> result == b)]
#[ensures( b > a ==> result == a)]
#[ensures( a == b ==> result == b)]
fn min(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

impl ClampTransform {
    #[ensures(result.bounds === bounds)]
    fn make_clamp(bounds: Bounds) -> Self {
        Self { bounds }
    }

    // #[pure]
    // fn do_transform(self, data: i32) -> i32 {
    //     if data < self.bounds.lower {
    //         self.bounds.lower
    //     } else if data > self.bounds.upper {
    //         self.bounds.upper
    //     } else {
    //         data
    //     }
    // }

    #[pure]
    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(data < self.bounds.lower ==> result == self.bounds.lower)]
    #[ensures(data > self.bounds.upper ==> result == self.bounds.upper)]
    #[ensures(result <= self.bounds.upper)]
    #[ensures(result >= self.bounds.lower)]
    fn do_transform(self, data: i32) -> i32 {
        // implemented like this due to limitation in pure controll flow
        max(min(self.bounds.upper, data), self.bounds.lower)
    }

    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(result.0 == self.do_transform(data))]
    #[ensures(result.1 === self)]
    fn do_transform_impure(self, data: i32) -> (i32, Self) {
        if data < self.bounds.lower {
            (self.bounds.lower, self)
        } else if data > self.bounds.upper {
            (self.bounds.lower, self)
        } else {
            (data, self)

        }
    }
}



#[requires(a == b)]
fn assert_eq(a: i32, b: i32) {}


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

#[requires(forall(|i: i32| (0<= i && i< res.len) ==> between(res.get(i), 100, 200)))]
fn final_assert(res: Vector) {

}

fn client(data: Vector) {
    let t = ClampTransform::make_clamp(Bounds{ lower: 100, upper: 200 });
    let res = apply_row_by_row(t, data);
    final_assert(res);
    //prusti_assert!(forall(|i: i32| (0<= i && i< res.len) ==> res.get(i) <= 200 && res.get(i) >= 100))
}

fn main() {}
