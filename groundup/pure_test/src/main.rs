extern crate prusti_contracts;
use prusti_contracts::*;

fn return_a_bool() -> bool {
    true
}

#[pure]
#[ensures(result >= 0)]
#[trusted]
fn f1() -> i32 {
    1 + 8
}

#[pure]
#[ensures(result == i)]
fn id(i: i32) -> i32 {
    i
}

#[pure]
fn idb(i: bool) -> bool {
    i
}

#[requires(i >= 0)]
#[ensures(result >= i)]
#[pure]
fn len2(i: i32) -> i32 {
    i
}

#[requires(_i >= 0)]
fn assert_ge_0(_i: i32) {}



#[requires(a == b)]
fn assert_eq(a: i32, b: i32) {}


#[pure]
fn t() -> bool { true }
#[pure]
fn f() -> bool { false }

#[ensures(true || true)]
fn bor1() { }


#[pure]
fn bor2() -> bool { true || true }


struct Vector {
    len: i32,
}

#[pure]
fn len_free(v: Vector) -> i32 {
    v.len
}

impl Vector {
    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    fn get(self, idx: i32) -> i32 {
        unimplemented!()
    }

    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.len)]
    #[ensures(result.0 == self.get(idx))]
    #[ensures(result.1.veq(self))]
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

    #[trusted]
    #[pure]
    #[ensures(result ==> (self.len == other.len))]
    #[ensures(result ==> forall(|i : i32| (i >= 0  && i < self.len) ==> other.get(i) == self.get(i)))]
    fn veq(self, other: Vector) -> bool {   
        unimplemented!()
    }

}


#[requires(vec.len == 10)]
fn vector_client(vec: Vector) {
    let vec = vec.set(5, 42);
    let res = vec.get(5);
    assert_eq(res, 42);
}


#[pure]
fn not(b: bool) -> bool {
    !b
}

fn main() {
    assert_ge_0(id(0));
    assert_ge_0(id(f1()));

    assert_ge_0(if idb(true) { 100 } else { -100 });

    let x = f1();
    let y = len2(x);
    let z = id(y);
    assert_ge_0(z);
}

