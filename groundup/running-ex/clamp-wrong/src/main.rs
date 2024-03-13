extern crate prusti_contracts;
use prusti_contracts::*;



#[requires(min <= max)]
#[ensures(result <= max)]
fn clamp(value: i32, min: i32, max: i32) -> i32 {
	if value < min { min }
	else if value > max { value }
	else { max }
}



fn main() {

}
