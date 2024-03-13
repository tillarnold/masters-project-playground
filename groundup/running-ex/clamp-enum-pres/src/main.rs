extern crate prusti_contracts;
use prusti_contracts::*;


struct ClampTransform {
    bounds: Bounds,
}

struct Bounds {
    min: i32,
    max: i32,
}

enum FallibleI32 {
    Ok(i32),
    Err,
}

#[trusted]
#[pure]
#[requires(false)]
fn unreachable_i32() -> i32 {
    unreachable!()
}

impl FallibleI32 {
    #[pure]
    #[requires(matches!(self, FallibleI32::Ok(_)))]
    fn unwrap_i32(self) -> i32 {
        match self {
            FallibleI32::Ok(val) => val,
            FallibleI32::Err => unreachable_i32(),
        }
    }
}


impl ClampTransform {
    #[pure]
    #[requires(self.bounds.min <= self.bounds.max)]
    fn clamp(self, data: i32) -> i32 {
        if data < self.bounds.min {
            (self.bounds.min)
        } else if data > self.bounds.max {
            (self.bounds.max)
        } else {
            (data)
        }
    }

    #[ensures(self.bounds.min <= self.bounds.max <==> matches!(result.0, FallibleI32::Ok(_)))]
    #[ensures(self.bounds.min <= self.bounds.max ==> result.0.unwrap_i32() === self.clamp(data))]
    #[ensures(result.1 === self)]
    fn clamp_impure(self, data: i32) -> (FallibleI32, Self) {
        if self.bounds.min > self.bounds.max {
            (FallibleI32::Err, self)
        } else if data < self.bounds.min {
            (FallibleI32::Ok(self.bounds.min), self)
        } else if data > self.bounds.max {
            (FallibleI32::Ok(self.bounds.max), self)
        } else {
            (FallibleI32::Ok(data), self)
        }
    }
}


fn main() {}
