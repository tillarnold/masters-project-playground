use std::{fmt::Debug, rc::Rc};

pub type Fallible<T> = Result<T, ()>;

macro_rules! fallible {
    ($variant:ident) => {
        Err(())
    };
}

pub struct Function {
    pub function: Rc<dyn Fn(&Vec<i32>) -> Fallible<Vec<i32>>>,
}
impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            function: self.function.clone(),
        }
    }
}

impl Function {
    fn new_fallible(function: impl Fn(&Vec<i32>) -> Fallible<Vec<i32>> + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    fn eval(&self, arg: &Vec<i32>) -> Fallible<Vec<i32>> {
        (self.function)(arg)
    }
}

struct StabilityMap(Rc<dyn Fn(u32) -> u32>);

impl StabilityMap {
    pub fn new_from_constant(c: u32) -> Self {
        StabilityMap(Rc::new(move |d_in: u32| (d_in.clone() * (c))))
    }
    pub fn eval(&self, input_distance: u32) -> u32 {
        (self.0)(input_distance)
    }
}

fn clamp(value: i32, min: i32, max: i32) -> Fallible<i32> {
    if min > max {
        return fallible!(FailedFunction);
    }
    Ok(if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    })
}

pub struct Transformation {
    pub input_domain: VectorDomain,
    pub output_domain: VectorDomain,
    function: Function,
    stability_map: StabilityMap,
}

impl Transformation {
    pub fn invoke(&self, arg: &Vec<i32>) -> Fallible<Vec<i32>> {
        self.function.eval(arg)
    }

    pub fn map(&self, d_in: u32) -> u32 {
        self.stability_map.eval(d_in)
    }
}



fn apply_rows(
    value: &Vec<i32>,
    row_function: &impl Fn(&i32) -> Fallible<i32>,
) -> Fallible<Vec<i32>> {
    value.iter().map(row_function).collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bounds {
    min: i32,
    max: i32,
}

impl Bounds {
    pub fn member(&self, val: i32) -> bool {
        self.min <= val && val <= self.max
    }
}


pub trait Domain: Clone + PartialEq + Debug {
    type Carrier;
    fn member(&self, val: &Self::Carrier) -> bool;
}


#[derive(Clone, PartialEq, Debug)]
pub struct AtomDomain {
    pub bounds: Option<Bounds>,
}

impl Domain for AtomDomain {
    type Carrier = i32;
    fn member(&self, val: &i32) -> bool {
        self.bounds.clone().map(|b| b.member(*val)).unwrap_or(false)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VectorDomain {
    pub element_domain: AtomDomain,
}


impl Domain for VectorDomain {
    type Carrier = Vec<i32>;
    fn member(&self, val: &Vec<i32>) -> bool {
        for e in val {
            if !self.element_domain.member(e) {
                return false;
            }
        }
       
        true
    }
}

pub fn make_row_by_row_fallible(
    input_domain: VectorDomain,
    output_row_domain: AtomDomain,
    row_function: impl 'static + Fn(&i32) -> Fallible<i32>,
) -> Transformation {
    let output_domain = VectorDomain {
        element_domain: output_row_domain,
    };
    Transformation {
        input_domain,
        output_domain,
        function: Function::new_fallible(move |arg: &Vec<i32>| apply_rows(arg, &row_function)),
        stability_map: StabilityMap::new_from_constant(1),
    }
}

pub fn make_clamp(input_domain: VectorDomain, bounds: Bounds) -> Transformation {
    let mut output_row_domain = input_domain.element_domain.clone();
    output_row_domain.bounds = Some(bounds.clone());

    make_row_by_row_fallible(input_domain, output_row_domain, move |arg: &i32| {
        clamp(arg.clone(), bounds.min.clone(), bounds.max.clone())
    })
}

fn example_client() -> Fallible<()> {
    let input_domain = VectorDomain{ element_domain : AtomDomain { bounds: None }};
    let clamp_transform = make_clamp(input_domain, Bounds { min: 10, max: 20 });

    println!("privacy spend {:?}", clamp_transform.map(1));

    let res = clamp_transform.invoke(&vec![1, 2, 3, 15, 100])?;

    println!("{:?}", res);

    Ok(())
}

fn main() {
    example_client().unwrap();
}
