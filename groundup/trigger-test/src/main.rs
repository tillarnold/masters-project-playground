extern crate prusti_contracts;
use prusti_contracts::*;

struct Foo {
    min: i32,
    max: i32,
}


#[pure]
fn property(f: Foo, i: i32) -> bool{
    f.min == 100
}

#[ensures(forall(|i: i32| property(result, i), triggers=[(property(result, i),)]))]
fn foo() -> Foo {
    Foo {
        min: 100,
        max: 100,
    }
}


fn main() {}
