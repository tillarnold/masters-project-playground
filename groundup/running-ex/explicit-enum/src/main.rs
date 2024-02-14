extern crate prusti_contracts;
use prusti_contracts::*;

#[repr(i8)]
enum Color {
    Red(i32, i32, i32) = 42,
    Purple = 3,
    Green = -8,
}


fn foo(a: Color) {

}
fn main() {}
