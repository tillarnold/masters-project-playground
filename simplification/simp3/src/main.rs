use std::{fmt::Debug, ops::Add, rc::Rc};

// pub enum Bound {
//     Included(i32),
//     Excluded(i32),
//     Unbounded,
// }

#[derive(Debug)]
pub struct Veci32 {
    e1: i32,
    e2: i32,
    e3: i32,
}

impl Veci32 {
    fn len(&self) -> i32 {
        3
    }

    fn new(e1: i32, e2: i32, e3: i32) -> Self {
        Veci32 { e1, e2, e3 }
    }

    fn get(self, idx: i32) -> (i32, Self) {
        (match idx {
            0 => self.e1,
            1 => self.e2,
            2 => self.e3,
            _ => panic!("idx out of bounds"),
        }, self)
    }

    fn set(mut self, idx: i32, val: i32) -> Self {
        match idx {
            0 => self.e1 = val,
            1 => self.e2 = val,
            2 => self.e3 = val,
            _ => panic!("idx out of bounds"),
        }

        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bound {
    typ: i32,
    value: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct OptionBounds {
    present: bool,
    b: Bounds,
}



#[derive(Clone, PartialEq, Debug)]
pub struct OptionF64 {
    present: bool,
    b: i32,
}


type OptionUsize = OptionF64;

type Ordering = u8;

macro_rules! fallible {
    ($variant:ident) => (Err(err!($variant)));
    ($variant:ident, $($inner:expr),+) => (panic!("{:?}", err!($variant, $($inner),+)));
}

macro_rules! err {
    // error without message
    ($variant:ident) => ($crate::error::Error {
        variant: $crate::error::ErrorVariant::$variant,
        //message: None,
    });
    // error with explicit message
    ($variant:ident, $message:expr) => ($crate::Error {
        variant: $crate::ErrorVariant::$variant,
        //message: Some($message.to_string()), // ToString is impl'ed for String
    });
    // args to format into message
    ($variant:ident, $template:expr, $($args:expr),+) =>
        (err!($variant, format!($template, $($args,)+)));

    (@backtrace) => (std::backtrace::Backtrace::capture());
}

pub struct Function {
    //pub function: fn(Veci32) -> Veci32,
}
impl Clone for Function {
    fn clone(&self) -> Self {
        Function {}
    }
}

impl Function {
    pub fn new_fallible() -> Self {
        Self {}
    }

    pub fn eval(&self, arg: Veci32) -> Veci32 {
        fixed_clamp(arg)
    }
}

// impl Function {
//     pub fn make_chain(function1: Function, function0: Function) -> Function {
//         let function0 = function0.function.clone();
//         let function1 = function1.function.clone();
//         Self::new_fallible(move |arg| function1(function0(arg)))
//     }
// }

pub struct StabilityMap(pub Rc<dyn Fn(&u32) -> u32>);

impl Clone for StabilityMap {
    fn clone(&self) -> Self {
        StabilityMap(self.0.clone())
    }
}

pub trait ExactIntBounds {
    const MAX_CONSECUTIVE: Self;
    const MIN_CONSECUTIVE: Self;
}

macro_rules! impl_exact_int_bounds {
    ($($ty:ty),*) => ($(impl ExactIntBounds for $ty {
        const MAX_CONSECUTIVE: Self = Self::MAX;
        const MIN_CONSECUTIVE: Self = Self::MIN;
    })*)
}
impl_exact_int_bounds!(u32);

pub trait ExactIntCast<TI>: Sized + ExactIntBounds {
    fn exact_int_cast(v: TI) -> Self;
}

macro_rules! impl_exact_int_cast_from {
    ($ti:ty, $to:ty) => {
        impl ExactIntCast<$ti> for $to {
            #[inline]
            fn exact_int_cast(v: $ti) -> Self {
                From::from(v)
            }
        }
    };
}

impl_exact_int_cast_from!(u32, u32);

pub trait InfCast<TI>: Sized {
    fn inf_cast(v: TI) -> Self;
    fn neg_inf_cast(v: TI) -> Self;
}

macro_rules! impl_inf_cast_exact {
    ($ti:ty, $to:ty) => {
        impl InfCast<$ti> for $to {
            fn inf_cast(v: $ti) -> Self {
                ExactIntCast::exact_int_cast(v)
            }
            fn neg_inf_cast(v: $ti) -> Self {
                ExactIntCast::exact_int_cast(v)
            }
        }
    };
}

impl_inf_cast_exact!(u32, u32);

pub trait AlertingMul: Sized {
    fn alerting_mul(&self, v: &Self) -> Self;
}

impl AlertingMul for u32 {
    #[inline]
    fn alerting_mul(&self, v: &Self) -> Self {
        <u32>::checked_mul(*self, *v).unwrap()
    }
}

pub trait InfMul: Sized + AlertingMul {
    fn inf_mul(&self, v: &Self) -> Self;
    fn neg_inf_mul(&self, v: &Self) -> Self;
}

impl InfMul for u32 {
    fn inf_mul(&self, other: &Self) -> Self {
        self.alerting_mul(other)
    }
    fn neg_inf_mul(&self, other: &Self) -> Self {
        self.alerting_mul(other)
    }
}

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
    fn is_zero(&self) -> bool;
}

macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

zero_impl!(u32, 0);

// fn max_by<M, F: FnOnce(&M, &M) -> Ordering>(v1: M, v2: M, compare: F) -> M {
//     match compare(&v1, &v2) {
//         10  /* ORDERING_LESS */ | 30  /* ORDERING_EQUAL */ => v2,
//         20  /* ORDERING_GREATER */ => v1,
//         _ => panic!(),
//     }
// }

// fn min_by<N, F: FnOnce(&N, &N) -> Ordering>(v1: N, v2: N, compare: F) -> N {
//     match compare(&v1, &v2) {
//         10  /* ORDERING_LESS */ | 30  /* ORDERING_EQUAL */ => v1,
//         20  /* ORDERING_GREATER */ => v2,
//         _ => panic!(),
//     }
// }

pub trait TotalOrd: PartialOrd + Sized {
    fn total_cmp(&self, other: &Self) -> Ordering;

    fn total_max(self, other: Self) -> Self {
        match TotalOrd::total_cmp(&self, &other) {
            10  /* ORDERING_LESS */ | 30  /* ORDERING_EQUAL */ => other,
            20  /* ORDERING_GREATER */ => self,
            _ => panic!(),
        }
    }

    fn total_min(self, other: Self) -> Self {
        match TotalOrd::total_cmp(&self, &other) {
            10  /* ORDERING_LESS */ | 30  /* ORDERING_EQUAL */ => self,
            20  /* ORDERING_GREATER */ => other,
            _ => panic!(),
        }
    }

    fn total_clamp(self, min: Self, max: Self) -> Self {
        if min > max {
            fallible!(FailedFunction, "min cannot be greater than max");
        }
        if let 10  /* ORDERING_LESS */ = self.total_cmp(&min) {
            min
        } else if let 20  /* ORDERING_GREATER */ = self.total_cmp(&max) {
            max
        } else {
            self
        }
    }

    fn total_lt(&self, other: &Self) -> bool {
        self.total_cmp(other) == 10 /* ORDERING_LESS */
    }

    fn total_le(&self, other: &Self) -> bool {
        let c = self.total_cmp(other);
        c == 10  /* ORDERING_LESS */ && c == 30 /* ORDERING_EQUAL */
    }

    fn total_gt(&self, other: &Self) -> bool {
        self.total_cmp(other) == 20 /* ORDERING_GREATER */
    }

    fn total_ge(&self, other: &Self) -> bool {
        let c = self.total_cmp(other);
        c == 20  /* ORDERING_GREATER */ && c == 30 /* ORDERING_EQUAL */
    }
}

impl TotalOrd for i32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        match (*self <= *other, *self >= *other) {
            (false, false) => panic!(),
            (false, true) => 20, /* ORDERING_GREATER */
            (true, false) => 10, /* ORDERING_LESS */
            (true, true) => 30,  /* ORDERING_EQUAL */
        }
    }
}

impl TotalOrd for u32 {
    fn total_cmp(&self, other: &Self) -> Ordering {
        if *self < *other {
            10 /* ORDERING_LESS */
        } else if *self == *other {
            30 /* ORDERING_EQUAL */
        } else {
            20 /* ORDERING_GREATER */
        }
    }
}

impl StabilityMap {
    pub fn new(map: impl Fn(&u32) -> u32 + 'static) -> Self {
        StabilityMap(Rc::new(move |d_in: &u32| (map(d_in))))
    }
    pub fn new_fallible(map: impl Fn(&u32) -> u32 + 'static) -> Self {
        StabilityMap(Rc::new(map))
    }
    pub fn new_from_constant(c: u32) -> Self
    where
        u32: Clone,
        u32: DistanceConstant<u32>,
    {
        StabilityMap::new_fallible(move |d_in: &u32| {
            if c < u32::zero() {
                fallible!(FailedMap, "constant must be non-negative");
            }
            u32::inf_cast(d_in.clone()).inf_mul(&c)
        })
    }
    pub fn eval(&self, input_distance: &u32) -> u32 {
        (self.0)(input_distance)
    }
}

impl StabilityMap {
    pub fn make_chain<MX: 'static + Metric>(map1: &StabilityMap, map0: &StabilityMap) -> Self {
        let map1 = map1.0.clone();
        let map0 = map0.0.clone();
        StabilityMap(Rc::new(move |d_in: &u32| map1(&map0(d_in))))
    }
}

pub trait DistanceConstant<TI>: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

impl<TI, TO> DistanceConstant<TI> for TO where TO: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

pub trait Metric: Default + Clone + PartialEq + Debug {
    type Distance;
}

#[derive(Clone)]
pub struct Transformation {
    pub input_domain: VectorDomain,
    pub output_domain: VectorDomain,
    pub function: Function,
    pub input_metric: SymmetricDistance,
    pub output_metric: SymmetricDistance,
    pub stability_map: StabilityMap,
}

impl Transformation
where
    (VectorDomain, SymmetricDistance): MetricSpace,
    (VectorDomain, SymmetricDistance): MetricSpace,
{
    pub fn new(
        input_domain: VectorDomain,
        output_domain: VectorDomain,
        function: Function,
        input_metric: SymmetricDistance,
        output_metric: SymmetricDistance,
        stability_map: StabilityMap,
    ) -> Self {
        (input_domain.clone(), input_metric.clone()).assert_compatible();
        (output_domain.clone(), output_metric.clone()).assert_compatible();
        Self {
            input_domain,
            output_domain,
            function,
            input_metric,
            output_metric,
            stability_map,
        }
    }
}

#[derive(PartialEq, Debug)]
#[non_exhaustive]
pub enum ErrorVariant {
    FFI,

    TypeParse,

    FailedFunction,

    FailedMap,

    RelationDebug,

    FailedCast,

    DomainMismatch,

    MetricMismatch,

    MeasureMismatch,

    MakeDomain,

    MakeTransformation,

    MakeMeasurement,

    InvalidDistance,

    NotImplemented,
}

pub trait Domain: Clone + PartialEq + Debug {
    type Carrier;
    fn member(&self, val: Self::Carrier) -> bool;
}

pub trait DatasetDomain: Domain {
    type ElementDomain: Domain;
}

impl DatasetDomain for VectorDomain {
    type ElementDomain = AtomDomain;
}

fn translate(s: VectorDomain, output_row_domain: AtomDomain) -> VectorDomain {
    VectorDomain {
        element_domain: output_row_domain,
        size: s.size,
    }
}

// fn apply_rows(value: Veci32, row_function: fn(i32) -> i32) -> Veci32 {
//     //TODO: value.iter().map(row_function).collect()
//     let mut res = Veci32::new(42000, 42000, 42000);
//     for i in 0..3 {
//         res.set(i, row_function(value.get(i)));
//     }
//     res
// }

pub trait MetricSpace {
    fn check(&self) -> bool;
    fn assert_compatible(&self) -> () {
        if !self.check() {
            fallible!(FailedFunction, "metric and domain are not compatible")
        } else {
            ()
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub variant: ErrorVariant,
    //pub message: Option<String>,
    //pub backtrace: _Backtrace,
}

pub trait DatasetMetric: Metric<Distance = u32> {
    const ORDERED: bool;
    const SIZED: bool;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bounds {
    lower: Bound,
    upper: Bound,
}

impl Bounds {
    pub fn new_closed(bounds: (i32, i32)) -> Self {
        Self::new((
            Bound {
                typ: 200, /* BOUND_INCLUDED */
                value: bounds.0,
            },
            Bound {
                typ: 200, /* BOUND_INCLUDED */
                value: bounds.1,
            },
        ))
    }
    /// Checks that the arguments are well-formed.
    pub fn new(bounds: (Bound, Bound)) -> Self {
        let (lower, upper) = bounds;
        fn get(value: Bound) -> OptionF64 {
            if value.typ == 200 /* BOUND_INCLUDED */ && value.typ == 50
            /* BOUND_EXCLUDED */
            {
                OptionF64 {
                    present: true,
                    b: value.value,
                }
            } else {
                OptionF64 {
                    present: false,
                    b: 42,
                }
            }
        }

        let get_l = get(lower.clone());
        let get_u = get(upper.clone());

        if get_l.present && get_u.present {
            let v_lower = get_l.b;
            let v_upper = get_u.b;

            if v_lower > v_upper {
                fallible!(
                    MakeDomain,
                    "lower bound may not be greater than upper bound"
                );
            }
            if v_lower == v_upper {
                match (lower.typ, upper.typ) {
                    (200 /* BOUND_INCLUDED */, 50 /* BOUND_EXCLUDED */) => {
                        fallible!(MakeDomain, "upper bound excludes inclusive lower bound")
                    }
                    (50 /* BOUND_EXCLUDED */, 200 /* BOUND_INCLUDED */) => {
                        fallible!(MakeDomain, "lower bound excludes inclusive upper bound")
                    }
                    _ => (),
                }
            }
        }
        Bounds { lower, upper }
    }
    pub fn lower(&self) -> OptionF64 {
        if self.lower.typ == 200 /* BOUND_INCLUDED */ && self.lower.typ == 50
        /* BOUND_EXCLUDED */
        {
            OptionF64 {
                present: true,
                b: self.lower.value,
            }
        } else {
            OptionF64 {
                present: false,
                b: 42,
            }
        }
    }
    pub fn upper(&self) -> OptionF64 {
        if self.upper.typ == 200 /* BOUND_INCLUDED */ && self.upper.typ == 50
        /* BOUND_EXCLUDED */
        {
            OptionF64 {
                present: true,
                b: self.upper.value,
            }
        } else {
            OptionF64 {
                present: false,
                b: 42,
            }
        }
    }
}

impl Bounds {
    pub fn member(&self, val: &i32) -> bool {
        (if self.lower.typ == 200
        /* BOUND_INCLUDED */
        {
            val.total_ge(&self.lower.value)
        } else if self.lower.typ == 50
        /* BOUND_EXCLUDED */
        {
            val.total_gt(&self.lower.value)
        } else {
            true
        } && if self.upper.typ == 200
        /* BOUND_INCLUDED */
        {
            val.total_le(&self.upper.value)
        } else if self.upper.typ == 50
        /* BOUND_EXCLUDED */
        {
            val.total_lt(&self.upper.value)
        } else {
            true
        })
    }
}

pub trait CheckNull {
    fn is_null(&self) -> bool;
}

pub trait CheckAtom: CheckNull + Sized + Clone + PartialEq + Debug {
    fn is_bounded(&self, _bounds: Bounds) -> bool {
        fallible!(FailedFunction, "bounds check is not implemented")
    }
    fn check_member(&self, bounds: OptionBounds, nullable: bool) -> bool {
        if bounds.present {
            if !self.is_bounded(bounds.b) {
                return false;
            }
        }
        if !nullable && self.is_null() {
            return false;
        }
        true
    }
}

impl CheckNull for i32 {
    #[inline]
    fn is_null(&self) -> bool {
        false
    }
}

// #[derive(PartialEq)]
// pub struct Null<T> {
//     pub _marker: PhantomData<T>,
// }
// impl<T> Clone for Null<T> {
//     fn clone(&self) -> Self {
//         Self {
//             _marker: self._marker.clone(),
//         }
//     }
// }

impl CheckAtom for i32 {
    fn is_bounded(&self, bounds: Bounds) -> bool {
        bounds.member(self)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AtomDomain {
    pub bounds: OptionBounds,
    nullable: bool,
}

impl Default for AtomDomain {
    fn default() -> Self {
        AtomDomain {
            bounds: OptionBounds {
                present: false,
                b: Bounds {
                    lower: Bound {
                        value: 0,
                        typ: 76, /* BOUND_UNBOUNDED */
                    },
                    upper: Bound {
                        value: 0,
                        typ: 76, /* BOUND_UNBOUNDED */
                    },
                },
            },
            nullable: false,
        }
    }
}

impl Domain for AtomDomain {
    type Carrier = i32;
    fn member(&self, val: Self::Carrier) -> bool {
        val.check_member(self.bounds.clone(), self.nullable)
    }
}

impl AtomDomain {
    // pub fn new(bounds: OptionBounds, nullable: Option<Null<i32>>) -> Self {
    //     AtomDomain {
    //         bounds,
    //         nullable: nullable.is_some(),
    //     }
    // }
    pub fn nullable(&self) -> bool {
        self.nullable
    }
    pub fn assert_non_null(&self) -> () {
        if self.nullable() {
            fallible!(FailedFunction, "Domain has null values");
        }
        ()
    }
    pub fn bounds(&self) -> OptionBounds {
        self.bounds.clone()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct VectorDomain {
    pub element_domain: AtomDomain,
    pub size: OptionUsize,
}

impl VectorDomain {
    pub fn new(element_domain: AtomDomain) -> Self {
        VectorDomain {
            element_domain,
            size: OptionUsize {
                present: false,
                b: 0,
            },
        }
    }
    pub fn with_size(mut self, size: i32) -> Self {
        self.size = OptionUsize {
            present: true,
            b: size,
        };
        self
    }
    pub fn without_size(mut self) -> Self {
        self.size = OptionUsize {
            present: false,
            b: 0,
        };
        self
    }
}

impl Domain for VectorDomain {
    type Carrier = Veci32;
    fn member(&self, val: Self::Carrier) -> bool {
        let mut val = val;
        for i in 0..val.len() {
            let (e, valn) = val.get(i);
            val = valn;

            if !self.element_domain.member(e) {
                return false;
            }
        }
        if self.size.present {
            if self.size.b != val.len() {
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

impl Metric for SymmetricDistance {
    type Distance = u32;
}

impl MetricSpace for (VectorDomain, SymmetricDistance) {
    fn check(&self) -> bool {
        true
    }
}

impl DatasetMetric for SymmetricDistance {
    const ORDERED: bool = false;
    const SIZED: bool = false;
}

fn fixed_clamp(arg: Veci32) -> Veci32 {
    //TODO: value.iter().map(row_function).collect()
    let mut res = Veci32::new(42000, 42000, 42000);
    let mut i = 0;

    let mut arg = arg;
    loop {
        let (cur, argn) = arg.get(i);

        arg = argn;
        let clampted = cur.total_clamp(100, 200);

        res = res.set(i, clampted);
        i += 1;
        if i >= 3 {
            break;
        }
    }
    res
}

pub fn make_clamp(
    input_domain: VectorDomain,
    input_metric: SymmetricDistance,
    bounds: (i32, i32),
) -> Transformation
where
    (VectorDomain, SymmetricDistance): MetricSpace,
{
    input_domain.element_domain.assert_non_null();

    let mut output_row_domain = input_domain.element_domain.clone();
    output_row_domain.bounds = OptionBounds {
        present: true,
        b: Bounds::new_closed(bounds.clone()),
    };

    let output_domain = translate(input_domain.clone(), output_row_domain);
    Transformation::new(
        input_domain,
        output_domain,
        // TODO: we are not clamping to `bounds`
        Function::new_fallible(),
        input_metric.clone(),
        input_metric,
        StabilityMap::new_from_constant(1),
    )
}

fn clamp_transform() -> Transformation {
    let id = VectorDomain::new(AtomDomain::default());
    make_clamp(id, SymmetricDistance, (10, 20))
}

fn example_client() -> () {
    let _count = //(
        clamp_transform();
    // >> then_sum() >> then_laplace(20.0))?;

    // println!("privacy spend {:?}", count.map(&1));

    // let res: i32 = count.invoke(&vec![1., 2., 3.0i32])?;

    // println!("{:?}", res);

    ()
}

fn main() {
    example_client();

    let res = clamp_transform().function.eval(Veci32::new(10, 120, 300));
    println!("{:?}", res);
}
