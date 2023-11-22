
extern crate prusti_contracts;
use prusti_contracts::*;

#[repr(i16)]
enum Direction {
    Up = -100,
    Down = 8,
    Left = -23123,
    Right= 0,
}


#[pure]
#[trusted]
fn eq(s: Direction, other: Direction) -> bool {
    match (s, other) {
        (Direction::Up, Direction::Up) => true,
        (Direction::Down, Direction::Down) => true,
        (Direction::Left, Direction::Left) => true,
        (Direction::Right, Direction::Right) => true,
        _unused => false,
    }
}

impl Direction {
    #[ensures(!(result === self))]
    fn inverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    #[ensures((result === self))]
    fn id(self) -> Direction {
        match self {
            Direction::Up => Direction::Up,
            Direction::Down => Direction::Down,
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}

#[requires(b)]
fn assert(b: bool) {}

#[requires(a === b)]
fn assert_eq_dir(a: Direction, b: Direction) {}


#[requires(!(a === b))]
fn assert_neq_dir(a: Direction, b: Direction) {}

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

impl IntOption {
    #[pure]
    #[trusted]
    #[ensures(result == (self === IntOption::None))]
    fn is_none(self) -> bool {
        matches!(self, IntOption::None)
    }
    #[pure]
    #[trusted]
    #[ensures(result == !(self.is_none()))]
    fn is_some(self) -> bool {
        matches!(self, IntOption::Some(..))
    }

    #[ensures(self.is_none() ==> result == default)]
    #[ensures(self === IntOption::None ==> result == default)]
    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            IntOption::Some(v) => v,
            IntOption::None => default,
        }
    }

    #[pure]
    fn none() -> IntOption {
        IntOption::None
    }

    #[pure]
    fn some(i: i32) -> IntOption {
        IntOption::Some(i)
    }
}


fn f3() {
    match IntOption::Some(3) {
        IntOption::None => assert(false),
        IntOption::Some(v) => assert(v == 3),
    }
}

fn main() {
    let x = Person {
        name: 42,
        address: 9,
        facing: Direction::Up,
    };

    let x = Direction::Down;
    let y = x.inverse().id();
    assert_neq_dir(y, Direction::Down);

 

    let v = IntOption::None.unwrap_or(22);
    assert(v == 22);

    IntOption::none();
    IntOption::some(3);
}

struct Person {
    name: i32,
    address: u8,
    facing: Direction,
}
