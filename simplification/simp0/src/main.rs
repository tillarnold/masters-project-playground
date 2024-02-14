use std::{fmt::Debug, marker::PhantomData, rc::Rc};

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
    pub fn new_fallible(function: impl Fn(&Vec<i32>) -> Fallible<Vec<i32>> + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    pub fn eval(&self, arg: &Vec<i32>) -> Fallible<Vec<i32>> {
        (self.function)(arg)
    }
}

pub struct StabilityMap(pub Rc<dyn Fn(u32) -> Fallible<u32>>);

impl Clone for StabilityMap {
    fn clone(&self) -> Self {
        StabilityMap(self.0.clone())
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

impl StabilityMap {
    pub fn new_from_constant(c: u32) -> Self {
        StabilityMap(Rc::new(move |d_in: u32| Ok(d_in.clone() * (c))))
    }
    pub fn eval(&self, input_distance: u32) -> Fallible<u32> {
        (self.0)(input_distance)
    }
}

pub trait Metric: Default + Clone + PartialEq + Debug {
    type Distance;
}

#[derive(Clone)]
pub struct Transformation {
    pub input_domain: VectorDomain<AtomDomain>,
    pub output_domain: VectorDomain<AtomDomain>,
    pub function: Function,
    pub stability_map: StabilityMap,
}

impl Transformation {
    pub fn new(
        input_domain: VectorDomain<AtomDomain>,
        output_domain: VectorDomain<AtomDomain>,
        function: Function,

        stability_map: StabilityMap,
    ) -> Fallible<Self> {
        Ok(Self {
            input_domain,
            output_domain,
            function,
            stability_map,
        })
    }

    pub fn invoke(
        &self,
        arg: &<VectorDomain<AtomDomain> as Domain>::Carrier,
    ) -> Fallible<<VectorDomain<AtomDomain> as Domain>::Carrier> {
        self.function.eval(arg)
    }

    pub fn map(&self, d_in: u32) -> Fallible<u32> {
        self.stability_map.eval(d_in)
    }
}

pub trait Domain: Clone + PartialEq + Debug {
    type Carrier;
    fn member(&self, val: &Self::Carrier) -> bool;
}

pub trait DatasetDomain: Domain {
    type ElementDomain: Domain;
}

impl<D: Domain> DatasetDomain for VectorDomain<D> {
    type ElementDomain = D;
}

fn translate<DIA: Domain, DOA: Domain>(
    s: &VectorDomain<DIA>,
    output_row_domain: <VectorDomain<DOA> as DatasetDomain>::ElementDomain,
) -> VectorDomain<DOA> {
    VectorDomain {
        element_domain: output_row_domain,
        size: s.size,
    }
}

fn apply_rows(
    value: &Vec<i32>,
    row_function: &impl Fn(&i32) -> Fallible<i32>,
) -> Fallible<Vec<i32>> {
    value.iter().map(row_function).collect()
}

pub trait MetricSpace {
    fn check(&self) -> bool;
    fn assert_compatible(&self) -> Fallible<()> {
        if !self.check() {
            fallible!(FailedFunction)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bounds {
    lower: i32,
    upper: i32,
}

impl Bounds {
    pub fn member(&self, val: i32) -> bool {
        self.lower <= val && val <= self.upper
    }
}

#[derive(PartialEq)]
pub struct Null<T> {
    pub _marker: PhantomData<T>,
}
impl<T> Clone for Null<T> {
    fn clone(&self) -> Self {
        Self {
            _marker: self._marker.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AtomDomain {
    pub bounds: Option<Bounds>,
    nullable: bool,
}

impl Default for AtomDomain {
    fn default() -> Self {
        AtomDomain {
            bounds: None,
            nullable: false,
        }
    }
}

impl Domain for AtomDomain {
    type Carrier = i32;
    fn member(&self, val: &Self::Carrier) -> bool {
        self.bounds.clone().map(|b| b.member(*val)).unwrap_or(false)
    }
}

impl AtomDomain {
    pub fn new(bounds: Option<Bounds>, nullable: Option<Null<f64>>) -> Self {
        AtomDomain {
            bounds,
            nullable: nullable.is_some(),
        }
    }
    pub fn nullable(&self) -> bool {
        self.nullable
    }
    pub fn assert_non_null(&self) -> Fallible<()> {
        if self.nullable() {
            return fallible!(FailedFunction);
        }
        Ok(())
    }
    pub fn bounds(&self) -> Option<&Bounds> {
        self.bounds.as_ref()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VectorDomain<D: Domain> {
    pub element_domain: D,
    pub size: Option<usize>,
}

impl<D: Domain> VectorDomain<D> {
    pub fn new(element_domain: D) -> Self {
        VectorDomain {
            element_domain,
            size: None,
        }
    }
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn without_size(mut self) -> Self {
        self.size = None;
        self
    }
}

impl<D: Domain> Domain for VectorDomain<D> {
    type Carrier = Vec<D::Carrier>;
    fn member(&self, val: &Self::Carrier) -> bool {
        for e in val {
            if !self.element_domain.member(e) {
                return false;
            }
        }
        if let Some(size) = self.size {
            if size != val.len() {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct SymmetricDistance;

impl Default for SymmetricDistance {
    fn default() -> Self {
        SymmetricDistance
    }
}

impl PartialEq for SymmetricDistance {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub fn make_row_by_row_fallible(
    input_domain: VectorDomain<AtomDomain>,
    output_row_domain: AtomDomain,
    row_function: impl 'static + Fn(&i32) -> Fallible<i32>,
) -> Fallible<Transformation> {
    let output_domain = translate(&input_domain, output_row_domain);
    Transformation::new(
        input_domain,
        output_domain,
        Function::new_fallible(move |arg: &Vec<i32>| apply_rows(arg, &row_function)),
        StabilityMap::new_from_constant(1),
    )
}

pub fn make_clamp(
    input_domain: VectorDomain<AtomDomain>,
    bounds: Bounds,
) -> Fallible<Transformation> {
    input_domain.element_domain.assert_non_null()?;

    let mut output_row_domain = input_domain.element_domain.clone();
    output_row_domain.bounds = Some(bounds.clone());

    make_row_by_row_fallible(input_domain, output_row_domain, move |arg: &i32| {
        clamp(arg.clone(), bounds.lower.clone(), bounds.upper.clone())
    })
}

fn example_client() -> Fallible<()> {
    let id = VectorDomain::new(AtomDomain::default());
    let clamp = make_clamp(
        id,
        Bounds {
            lower: 10,
            upper: 20,
        },
    )?;

    println!("privacy spend {:?}", clamp.map(1));

    let res = clamp.invoke(&vec![1, 2, 3, 15, 100])?;

    println!("{:?}", res);

    Ok(())
}

fn main() {
    example_client().unwrap();
}
