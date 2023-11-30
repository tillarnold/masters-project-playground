extern crate prusti_contracts;
use prusti_contracts::*;

pub struct Mix {
    color_1: Color,
    color_2: Color,
    factor: u32,
}


pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


#[ensures(result.color_1 === old(x.color_1))]
fn foo(x: Mix) -> Mix {
    x
}

#[ensures(old(x) === result)]
fn foo2(x: Mix) -> Mix {
    x
}

#[ensures(rel1(result.color_1.r) === rel0(result.color_1.r))]
fn foo3(mut x: Mix) -> Mix {
    x.color_1.r = 3;
    x
}

#[ensures(rel0(a) == rel1(a) ==> rel0(result) == rel1(result))]
//#[ensures(result == old(a))]
fn id_2(a: i32, b: i32) -> i32 {
    a
}

fn main() {

}
