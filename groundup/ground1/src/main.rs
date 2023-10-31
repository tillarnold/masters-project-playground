extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Vector {
    len: i32,
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

    #[pure]
    #[trusted]
    #[requires(self.len >= 0)]
    #[ensures(self === result.0)]
    #[ensures(self === result.1)]
    fn clone(self) -> (Self, Self) {
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
    #[requires(self.len >= 0)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1 === self)]
    fn impure_get(self, idx: i32) -> (i32, Self) {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[requires(self.len >= 0)]
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

#[requires(data.len >= 0)]
#[requires(transform.bounds.lower < transform.bounds.upper)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|ip: i32| (0<= ip && ip < data.len)  ==> result.0.get(ip) == transform.do_transform(data.get(ip))))]
fn apply_row_by_row(transform: ClampTransform, data: Vector) -> (Vector, ClampTransform) {
    //apply_row_by_row_rec(transform, data, 0)
    if data.len <= 0 {
        return (data,transform);
    }

    let l = data.len;
    (apply_row_by_row_rec(transform, data, l - 1),transform)
}

// #[requires(transform.bounds.lower < transform.bounds.upper)]
// #[requires(i >= 0)]
// #[requires(data.len >= 0)]
// #[requires(i <= data.len)]
// #[ensures(transform === transform)]
// #[ensures(result.len === data.len)]
// #[ensures(forall(|ip: i32| (ip >= 0 && i > ip && ip < data.len)  ==> result.get(ip) == data.get(ip)))]
// #[ensures(forall(|ip: i32| (i<= ip && ip < data.len)  ==> result.get(ip) == transform.do_transform(data.get(ip)) ))]
// fn apply_row_by_row_rec(transform: ClampTransform, data: Vector, i: i32) -> Vector {
//     if i >= data.len {
//         return data;
//     } else {
//         let (cur, data) = data.impure_get(i);
//         let new = transform.do_transform(cur);
//         let data = data.set(i, new);

//         let(check, data) = data.impure_get(i);
//         assert_eq(check, new);
//         assert_geq_0(i);
//         let next = i + 1;
//         assert_geq_0(next);
//         assert_true(next <= data.len);
//         apply_row_by_row_rec(transform, data, next)
//     }
// }

#[requires(transform.bounds.lower < transform.bounds.upper)]
#[requires(idx >= 0)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.len === data.len)]
#[ensures(forall(|i: i32| (i >= 0 && i > idx && i < data.len)  ==> result.get(i) == data.get(i)))]
#[ensures(result.get(idx) == transform.do_transform(data.get(idx)))]
#[ensures(result.get(0) == transform.do_transform(data.get(0)))]
#[ensures(forall(|i: i32| (i >= 0 && i <= idx && i < data.len)  ==> result.get(i) == transform.do_transform(data.get(i))))]
fn apply_row_by_row_rec(transform: ClampTransform, data: Vector, idx: i32) -> Vector {
    let modified = if idx >= 1 {
         apply_row_by_row_rec(transform, data, idx - 1)
    } 
    else {
        data
    };

    let (cur, data) = modified.impure_get(idx);
    let (new, _transform) = transform.do_transform_impure(cur);
    let modified = data.set(idx, new);
    modified
}

// #[requires(transform.bounds.lower < transform.bounds.upper)]
// #[requires(idx >= 0)]
// #[requires(idx <= data.len)]
// #[requires(data.len == target.len)]
// #[requires(data.len >= 0)]

// #[ensures(result.len == target.len)]

// #[ensures(((idx < data.len)) ==> result.get(idx) ==  transform.do_transform(data.get(idx)))]

// #[ensures(forall(|i: i32| (i >= 0 &&  i < data.len && i < idx )  ==> result.get(i) == data.get(i)))]

// #[requires(forall(|i: i32| (i >= 0 &&  i < data.len && i < idx )  ==> target.get(i) == transform.do_transform(data.get(i))))]
// #[ensures(forall(|i: i32| (i >= 0 &&  i < data.len  )  ==> result.get(i) == transform.do_transform(data.get(i))))]
// fn apply_row_by_row_rec2(transform: ClampTransform, data: Vector, target: Vector, idx: i32) -> Vector {
//     if idx >= data.len {
//         return target;
//     }

//     let (cur, data) = data.impure_get(idx);
//     let new = transform.do_transform(cur);
//     let target = target.set(idx, new);

//     assert_geq_0(idx);
//     let next = idx + 1;
//     assert_geq_0(next);
//     assert_true(next <= data.len);

//     apply_row_by_row_rec2(transform, data, target, next)
// }

// #[requires(forall(|ip: i32| (0<= ip && ip < data.len)  ==> data.get(ip) > 10))]
// #[ensures(result.len === (data).len)]
// #[ensures(forall(|ip: i32| (0<= ip && ip < data.len)  ==> result.get(ip) == max((data.get(ip)), 1 )))]
// fn id_v(data: Vector) -> Vector {
//    data
// }



#[requires(i >= 0)]
fn assert_geq_0(i: i32) {}

#[pure]
#[ensures( a > b ==> result === a)]
#[ensures( b > a ==> result === b)]
#[ensures( b >= a ==> result === b)]
#[ensures( a == b ==> result === b)]
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[pure]
#[ensures( a > b ==> result === b)]
#[ensures( b > a ==> result === a)]
#[ensures( a >= b ==> result === b)]
#[ensures( a == b ==> result === b)]
fn min(a: i32, b: i32) -> i32 {
    if a >= b {
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
    fn do_transform_orig(self, data: i32) -> i32 {
        if data < self.bounds.lower {
            self.bounds.lower
        } else if data > self.bounds.upper {
            self.bounds.upper
        } else {
            data
        }
    }

    #[pure]
    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(data < self.bounds.lower ==> result == self.bounds.lower)]
    #[ensures(data > self.bounds.upper ==> result == self.bounds.upper)]
    #[ensures(data >= self.bounds.lower && data <= self.bounds.upper ==> result == data)]
    #[ensures(result <= self.bounds.upper)]
    #[ensures(result >= self.bounds.lower)]
    #[ensures(between(result, self.bounds.lower, self.bounds.upper))]
    fn do_transform(self, data: i32) -> i32 {
        // implemented like this due to limitation in pure controll flow
        max(self.bounds.lower, min(self.bounds.upper, data))
    }

    #[requires(self.bounds.lower < self.bounds.upper)]
    #[ensures(data < self.bounds.lower ==> result.0 == self.bounds.lower)]
    #[ensures(data > self.bounds.upper ==> result.0 == self.bounds.upper)]
    #[ensures(data >= self.bounds.lower && data <= self.bounds.upper ==> result.0 == data)]
    #[ensures(result.0 <= self.bounds.upper)]
    #[ensures(result.0 >= self.bounds.lower)]
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

#[requires(forall(|i: i32| (0<= i && i< res.len) ==> between(res.get(i), lower, upper)))]
fn final_assert(res: Vector, lower: i32, upper: i32) {}

#[requires(data.len >= 10)]
#[requires(lower < upper)]
pub fn client(data: Vector, lower: i32, upper: i32) {
    let t = ClampTransform::make_clamp(Bounds {
        lower,
        upper,
    });
    let (res, t2) = apply_row_by_row(t, data);


    // let (val3, res) = res.impure_get(3);
    // assert_true(val3 <= t2.bounds.upper);


    final_assert(res, lower, upper);
    //prusti_assert!(forall(|i: i32| (0<= i && i< res.len) ==> res.get(i) <= 200 && res.get(i) >= 100))
}

#[requires(data.len >= 0)]
#[requires(idx >= 0 && idx < data.len)]
fn clone_client(data: Vector, idx: i32) {
    let (v1, v2) = data.clone();

    assert_eq(v1.impure_get(idx).0, v2.impure_get(idx).0)
    //prusti_assert!(forall(|i: i32| (0<= i && i< res.len) ==> res.get(i) <= 200 && res.get(i) >= 100))
}

fn main() {
}
