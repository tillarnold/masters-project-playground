extern crate prusti_contracts;
use prusti_contracts::*;


#[derive(Clone)]
struct Vector {f: i32}


impl Vector {
    #[pure]
    #[trusted]
    #[ensures(result >= 0)]
    fn v_len(self) -> i32 {
        100
    }

    #[pure]
    #[trusted]
    #[requires(idx >= 0)]
    #[requires(idx < self.v_len())]
    fn get(self, idx: i32) -> i32 {
        0
    }
}

#[derive(Clone, Copy)]
struct Bounds {
    upper: i32,
    lower: i32,
}

fn return_a_bool() ->  bool {
    true
}


#[requires(i >= 0)]
fn ge_0(i: i32) {
}

fn main() {

    let v = Vector{f: 0};
    let p = v.clone();
    ge_0(v.v_len());
}
