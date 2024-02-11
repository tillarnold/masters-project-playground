extern crate prusti_contracts;
use prusti_contracts::*;



pub struct Set {
    contents: u32,
}

impl Set {
    #[pure]
    #[trusted]
    fn contains(self, el: i32) -> bool {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[ensures(forall(|el: i32| (self.contains(el) ==>  (result.contains(el)  ) ) ))]
    #[ensures(forall(|el: i32| (other.contains(el) ==>  (result.contains(el)  ) ) ))]
    #[ensures(forall(|el: i32| (result.contains(el) <==>  (self.contains(el) | other.contains( el) ) ) ))]
    fn union(self, other: Set) -> Set {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    // #[requires(self.card() < 1000)]
    #[requires(! self.contains(new))]
    #[ensures({let x = Set::singleton(new); let p = x.union(self); result === p })]
    // #[ensures(result.card() == self.card() + 1)]
    fn set_insert(self, new: i32) -> Set {
        unimplemented!()
    }


    #[pure]
    #[trusted]
    fn card(self) -> usize {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[ensures(result.contains(el))]
    #[ensures(result.card() == 1)]
    #[ensures(forall(|e: i32| (result.contains(e) <==>  e == el ) ))]
    fn singleton(el: i32) -> Set{
        unimplemented!()
    }
}




fn union_client_1(s: Set) {
    assert_true(s.union(Set::singleton(3)).contains(3))
}

fn union_client_2(s: Set) {
    assert_true(Set::singleton(3).union(s).contains(3))
}



#[pure]
#[trusted]
#[ensures(result ==> (
    {
    let (m1,m2) = m.clone_map();
    m1.keys().card() == m2.values().card() 
    }
))]

#[ensures(result ==> (
    {
    let (m1,m2) = m.clone_map();
    let (m2,m3) = m2.clone_map();
    (forall(|v : i32|  (m1.values().contains(v) ==> m2.translate(m3.translate_invert(v)) == v)))
    }
))]

fn valid_mapping_owned(m: Map) -> bool
{
    unimplemented!()
}


#[pure]
#[trusted]
#[ensures(result ==> (
    {
    let (m1,m2) = m.clone_map();
    
    m1.keys().card() == m2.values().card() 
    }
))]
#[ensures(result ==> (m.keys().contains(k)))]
#[ensures(result ==> (m.values().contains(v)))]
#[ensures(result ==> (m.translate(k) == v))]
#[ensures(result ==> (m.translate_invert(v) == k))]
fn mapps(m: Map, k: i32, v: i32) -> bool {
    unimplemented!()

}

#[pure]
#[trusted]
#[requires( (
    {
    let (m1,m2) = m.clone_map();
    
    m1.keys().card() == m2.values().card() 
    }
))]
#[requires((m.keys().contains(k)))]
#[requires( (m.values().contains(v)))]
#[requires((m.translate(k) == v))]
#[requires((m.translate_invert(v) == k))]
fn assert_mapps(m: Map, k: i32, v: i32)  {
    unimplemented!()

}

struct Map {
    contents: u32,
}


impl Map {
    #[pure]
    #[trusted]
    fn keys(self) -> Set {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    fn values(self) -> Set {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    fn translate(self, el: i32) -> i32 {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    fn translate_invert(self, el: i32) -> i32 {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[requires(!(self.keys().contains(k)))]
    #[requires(!(self.values().contains(v)))]
    #[requires({let (self1,self2) = self.clone_map(); self1.keys().card() == self2.values().card()})]
    #[ensures((result.keys() === ((self.keys()).set_insert((k)))))]
    #[ensures((result.values() === ((self.values()).set_insert((v)))))]
    #[ensures(result.translate(k) == v)]
    #[ensures(result.translate_invert(v) == k)]
    #[ensures(forall(|kp: i32|  { let (self1,self2) = self.clone_map(); (self1.keys().contains(kp) && kp != k) ==> result.translate(kp) == self2.translate( kp) } ))]
    #[ensures(forall(|vp: i32| { let (self1,self2) = self.clone_map(); (self1.values().contains(vp) && vp != v) ==> result.translate_invert(vp) == self2.translate_invert(vp) } ))]
    fn insert(self, k: i32, v: i32) -> Map  {
        unimplemented!()

    }


    #[pure]
    #[trusted]
    #[ensures(result.0 === self)]
    #[ensures(result.1 === self)]
    fn clone_map(self) -> (Self, Self) {
        unimplemented!()
    }

  

}



#[requires(valid_mapping_owned(m))]
#[requires(!(m.keys().contains(k)))]
#[requires(!(m.values().contains(v)))]
// #[ensures(valid_mapping_owned(result))]
#[pure]
fn insert_client(m: Map, k: i32, v: i32)  -> Map 
 {
     let a = m.insert(k,v);
     let (a,b) = a.clone_map();
     assert_true(a.keys().contains(k));


    //  (assert_mapps(a, k, v));
     b
}


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
    upper: i32,
    lower: i32,
}

struct ClampTransform {
    bounds: Bounds,
}

#[requires(transform.bounds.lower <= transform.bounds.upper)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|ip: usize| (ip < data.len) ==> result.0.get(ip) == transform.do_transform(data.get(ip))))]
fn apply_row_by_row(transform: ClampTransform, data: Vector) -> (Vector, ClampTransform) {
    if data.len <= 0 {
        return (data, transform);
    }

    let l = data.len;
    apply_row_by_row_rec(transform, data, l - 1)
}

#[requires(transform.bounds.lower <= transform.bounds.upper)]
#[requires(data.len >= 1)]
#[requires(idx < data.len)]
#[ensures(result.0.len === data.len)]
#[ensures(result.1 === transform)]
#[ensures(forall(|i: usize| ((i > idx) && (i < data.len)) ==> result.0.get(i) == data.get(i)))]
#[ensures(forall(|i: usize| ((i <= idx) && (i < data.len)) ==> result.0.get(i) == transform.do_transform(data.get(i))))]
fn apply_row_by_row_rec(
    transform: ClampTransform,
    data: Vector, /*<i32>*/
    idx: usize,
) -> (Vector /*<i32>*/, ClampTransform) {
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

// #[ensures(rel0(a) == rel1(a) ==> rel0(result) == rel1(result))]
// fn id_2(a: i32, b: i32) -> i32 {
//     a
// }

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
    #[requires(self.bounds.lower <= self.bounds.upper)]
    fn do_transform_old(self, data: i32) -> i32 {
        // implemented like this due to limitation in pure controll flow
        max(self.bounds.lower, min(self.bounds.upper, data))
    }

    #[pure]
    #[requires(self.bounds.lower <= self.bounds.upper)]
    #[ensures(result == self.do_transform_old(data))]
    fn do_transform(self, data: i32) -> i32 {
        if data < self.bounds.lower {
            self.bounds.lower
        } else if data > self.bounds.upper {
            self.bounds.upper
        } else {
            data
        }
    }

    #[requires(self.bounds.lower <= self.bounds.upper)]
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
fn between(val: i32, lower: i32, upper: i32) -> bool {
    val <= upper && val >= lower
}

#[requires(forall(|i: usize| i< res.len ==> {let value = res.get(i); lower <= value && value <= upper }))]
fn final_assert(res: Vector, lower: i32, upper: i32) {}

#[requires(lower <= upper)]
pub fn client(data: Vector, lower: i32, upper: i32) {
    let t = ClampTransform::make_clamp(Bounds { lower, upper });
    let (res, t2) = apply_row_by_row(t, data);

    final_assert(res, lower, upper);
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
