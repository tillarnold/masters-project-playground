extern crate prusti_contracts;
use prusti_contracts::*;


struct Bounds {
    min: i32,
    max: i32,
}

struct Values {
    val_1: i32,
    val_2: i32,
}

fn clamp(mut value: Values, bounds: Bounds) -> Values{
    if value.val_1 < bounds.min {
        value.val_1= bounds.min
    }
    else if value.val_1 > bounds.max {
        value.val_1 = bounds.max
    }

    value
    
}

fn main() {

}
