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
    #[ensures(forall(|el: i32| (result.contains(el) <==>  (self.contains(el) | other.contains( el) ) ) ))]
    #[ensures(forall(|el: i32| ( self.contains(el) ==> result.contains(el) ) ))]
    #[ensures(forall(|el: i32| ( other.contains(el) ==> result.contains(el) ) ))]
    fn union(self, other: Set) -> Set {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[ensures(result.0 === self)]
    #[ensures(result.1 === self)]
    fn clone_set(self) -> (Self, Self) {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    // TODO 
    fn card(self) -> usize {
        unimplemented!()
    }

    #[pure]
    #[trusted]
    #[ensures(result.contains(el))]
    #[ensures(result.card() == 1)]
    #[ensures(forall(|e: i32| (result.contains(e) <==>  e == el ) ))]
    #[ensures(result.contains(el))]
    fn singleton(el: i32) -> Set{
        unimplemented!()
    }
}






// function empty_mapping() : Mapping 
//     ensures keys(result) == Set()
//     ensures values(result) == Set()


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
    #[ensures((result.keys() === ((self.keys()).union(Set::singleton(k)))))]
    #[ensures((result.values() === ((self.values()).union(Set::singleton(v)))))]
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


fn single_client(s: Set) {
    assert_true(Set::singleton(3).contains(3))
}

fn union_client(s: Set) {
    let b = s.union(Set::singleton(3));
    assert_true(b.contains(3))
}

// #[requires(valid_mapping_owned(m))]
// #[requires(!(m.keys().contains(k)))]
// #[requires(!(m.values().contains(v)))]
// #[ensures(valid_mapping_owned(result))]
// #[pure]
// fn insert_client(m: Map, k: i32, v: i32)  -> Map 
//  {
//      let a = m.insert(k,v);
//      let (a,b) = a.clone_map();
//      assert_true(a.keys().contains(k));

//     //  assert_true(mapps(a, k, v));
//      b
// }

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



fn main() {

}
