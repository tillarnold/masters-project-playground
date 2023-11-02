extern crate prusti_contracts;
use prusti_contracts::*;

enum Direction {
    Up,
    Down,
    Left,
    Right
}


impl Direction {
    fn inverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
} 



enum IntOption {
    Some(i32),
    None
}

fn main() {
    let x = Company {name: 42, address: 9};

    let x = Direction::Down;
    let y = x.inverse();
}


struct Company {
    name: i32,
    address: u8,
}