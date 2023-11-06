extern crate prusti_contracts;
use prusti_contracts::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
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

#[requires(b)]
fn assert(b: bool) {}

#[pure]
fn is_up(dir: Direction) -> bool {
    if let Direction::Up = dir {
        return true;
    }

    return false;
}

fn test2(dir: Direction) {
    let is_up = match dir {
        Direction::Up => true,
        _ => false,
    };

    let is_down = match dir {
        Direction::Down => true,
        _ => false,
    };

    assert(!is_up || !is_down);
}

fn test(dir: Direction) {
    let other = match dir {
        Direction::Up => Direction::Down,
        _ => Direction::Up,
    };

    match other {
        Direction::Left => assert(false),
        _ => {}
    };

    let is_up = match other {
        Direction::Up => true,
        _ => false,
    };

    let is_down = match other {
        Direction::Down => true,
        _ => false,
    };

    assert(!is_up || !is_down);
    assert(is_up || is_down);
}

enum IntOption {
    Some(i32),
    None,
}

fn main() {
    let x = Person {
        name: 42,
        address: 9,
    };

    let x = Direction::Down;
    let y = x.inverse();

    let p = IntOption::Some(3);

    match p {
        IntOption::None => assert(false),
        IntOption::Some(v) => assert(v == 3),
    }
}

struct Person {
    name: i32,
    address: u8,
}
