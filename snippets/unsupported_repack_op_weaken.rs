pub struct S;

pub struct V {
     a: i32,
     s: S,
}


fn t(v: V) -> V {
    V {
        a: 0,
        s: v.s,
    }
}

fn main() {}
