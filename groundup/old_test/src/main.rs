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

impl Color {
    #[pure]
    #[ensures(result.0 === self)]
    #[ensures(result.1 === self)]
    fn clone_color(self) -> (Self, Self) {
        ( Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }, Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        })
    }
}

struct Pair {
    left: u8,
    right: u8,
}
#[ensures( (rel0(arg) === rel1(arg)) ==> (rel0(&result) === rel1(&result))) ]
fn const_struct_fn(arg: u8, extra: u8) -> Pair {
    Pair {
        left: arg,
        right: arg,
    }
}

enum Res {
    Error(i32),
    Ok(Color)
}


// impl Res {
//     #[pure]
//     #[ensures(result.0 === self)]
//     #[ensures(result.1 === self)]
//     fn clone_res(self) -> (Self, Self) {
//         match self {
//             Res::Error(e) => {
//                 ( Res::Error(e),  Res::Error(e))
//             },
//             Res::Ok(color) => {
//                 let (c1, c2) = color.clone_color();
//                 ( Res::Ok(c1),  Res::Ok(c2))
//             }
//         }
//     }


//     #[pure]
//     #[requires(matches!(self, Res::Error(_)))]
//     #[trusted]
//     fn unwrap_err(&self) -> i32 {
//         match self {
//             Res::Error(c) => *c,
//             Res::Ok(_) => unreachable_i32(),
//         }
//     }
// }

#[requires(false)]
fn unreachable_i32() -> i32 {
    0
}

fn p1(data: i32, bound: i32) -> bool {
    false
}

#[pure]
fn small_int(arg: i32) -> bool {
    return arg >= -10_000 && arg <= 10_000
}





#[requires(small_int(data))]
#[requires(small_int(bound))]
#[requires(small_int(other_thing))]
#[ensures(
    ((rel0(bound) == rel1(bound)) & (rel0(other_thing) == rel1(other_thing))) ==>
        

    
    (
      
        match (rel0(&result), rel1(&result)) {
        (Res::Error(e0), Res::Error(e1)) => e0 === e1, 
        (Res::Ok(_), Res::Ok(_))=> true,
        _ => false
    }
)
   


)]
fn ind_err(data: i32, bound: i32, other_thing: i32) -> Res {

    if other_thing == bound {
        return Res::Error(9)
    }

    if other_thing == 100 {
        return Res::Error(other_thing + bound)
    }

    // if data == 3  {
    //     return Res::Error(10)
    // }


     if data == 22 {
         return Res::Ok(Color{r: 100, g: 200, b: 90, a: 42})
     }

    Res::Ok(Color{r: 0, g: 0, b: 0, a: 0})
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
fn id_2(a: i32, b: i32) -> i32 {
    a
}



#[ensures(rel0({let x = if 12 == 12 {a} else {b}; x}) == rel1(a) ==> rel0(result) == rel1(result))]
fn id_2_ind(a: i32, b: i32) -> i32 {
    a
}


fn main() {

}
