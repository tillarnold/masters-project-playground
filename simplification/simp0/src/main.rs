use std::{fmt::Debug, rc::Rc};

pub type Fallible<T> = Result<T, ()>;

pub struct Function(Rc<dyn Fn(&Vec<i32>) -> Fallible<Vec<i32>>>);

impl Clone for Function {
    fn clone(&self) -> Self {
        Function(self.0.clone())
    }
}

impl Function {
    fn new_fallible(function: impl Fn(&Vec<i32>) -> Fallible<Vec<i32>> + 'static) -> Self {
        Self(Rc::new(function))
    }
}

struct StabilityMap(Rc<dyn Fn(u32) -> u32>);

impl StabilityMap {
    fn new_from_constant(c: u32) -> Self {
        StabilityMap(Rc::new(move |d_in: u32| (d_in.clone() * (c))))
    }
}

fn clamp(value: i32, min: i32, max: i32) -> Fallible<i32> {
    if min > max {
        return Err(());
    }
    Ok(if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    })
}

struct Transformation {
    input_domain: VectorDomain,
    output_domain: VectorDomain,
    function: Function,
    stability_map: StabilityMap,
}

impl Transformation {
    fn invoke(&self, arg: &Vec<i32>) -> Fallible<Vec<i32>> {
        (self.function.0)(arg)
    }

    fn map(&self, d_in: u32) -> u32 {
        (self.stability_map.0)(d_in)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Bounds {
    min: i32,
    max: i32,
}

impl Bounds {
    fn member(&self, val: i32) -> bool {
        self.min <= val && val <= self.max
    }
}

#[derive(Clone)]
struct AtomDomain {
    bounds: Option<Bounds>,
}

impl AtomDomain {
    fn member(&self, val: &i32) -> bool {
        self.bounds.clone().map(|b| b.member(*val)).unwrap_or(true)
    }
}

#[derive(Clone)]
struct VectorDomain {
    element_domain: AtomDomain,
}

impl VectorDomain {
    fn member(&self, val: &Vec<i32>) -> bool {
        for e in val {
            if !self.element_domain.member(e) {
                return false;
            }
        }

        true
    }
}

fn make_row_by_row_fallible(
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
        function: Function::new_fallible(move |arg: &Vec<i32>| {
            arg.iter().map(&row_function).collect()
        }),
        stability_map: StabilityMap::new_from_constant(1),
    }
}

fn make_clamp(input_domain: VectorDomain, bounds: Bounds) -> Transformation {
    let output_row_domain = AtomDomain {
        bounds: Some(bounds.clone()),
    };

    make_row_by_row_fallible(input_domain, output_row_domain, move |arg: &i32| {
        clamp(arg.clone(), bounds.min.clone(), bounds.max.clone())
    })
}

fn example_client() -> Fallible<()> {
    let input_domain = VectorDomain {
        element_domain: AtomDomain { bounds: None },
    };
    let clamp_transform = make_clamp(input_domain, Bounds { min: 10, max: 20 });

    println!("privacy spend {:?}", clamp_transform.map(1));

    let data = vec![1, 2, 3, 15, 100];
    assert!(clamp_transform.input_domain.member(&data));

    let res = clamp_transform.invoke(&data)?;
    println!("{:?}", res);

    // Appropriate output domain
    assert!(clamp_transform.output_domain.member(&res));

    Ok(())
}

fn main() {
    example_client().unwrap();
}
