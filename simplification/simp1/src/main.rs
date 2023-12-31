use std::{
    cmp::Ordering,
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Bound},
    rc::Rc,
};

macro_rules! fallible {
    ($variant:ident) => (Err(err!($variant)));
    ($variant:ident, $($inner:expr),+) => (Err(err!($variant, $($inner),+)));
}

macro_rules! err {
    // error without message
    ($variant:ident) => ($crate::error::Error {
        variant: $crate::error::ErrorVariant::$variant,
        message: None,
    });
    // error with explicit message
    ($variant:ident, $message:expr) => ($crate::Error {
        variant: $crate::ErrorVariant::$variant,
        message: Some($message.to_string()), // ToString is impl'ed for String
    });
    // args to format into message
    ($variant:ident, $template:expr, $($args:expr),+) =>
        (err!($variant, format!($template, $($args,)+)));

    (@backtrace) => (std::backtrace::Backtrace::capture());
}

pub struct Function {
    pub function: Rc<dyn Fn(&Vec<f64>) -> Fallible<Vec<f64>>>,
}
impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            function: self.function.clone(),
        }
    }
}

impl Function {
    pub fn new(function: impl Fn(&Vec<f64>) -> Vec<f64> + 'static) -> Self {
        Self::new_fallible(move |arg| Ok(function(arg)))
    }

    pub fn new_fallible(function: impl Fn(&Vec<f64>) -> Fallible<Vec<f64>> + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    pub fn eval(&self, arg: &Vec<f64>) -> Fallible<Vec<f64>> {
        (self.function)(arg)
    }
}

impl Function{
    pub fn make_chain<TX: 'static>(
        function1: &Function,
        function0: &Function,
    ) -> Function{
        let function0 = function0.function.clone();
        let function1 = function1.function.clone();
        Self::new_fallible(move |arg| function1(&function0(arg)?))
    }
}

pub struct StabilityMap(
    pub Rc<dyn Fn(&IntDistance) -> Fallible<IntDistance>>,
);

impl Clone for StabilityMap{
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
    fn exact_int_cast(v: TI) -> Fallible<Self>;
}

macro_rules! impl_exact_int_cast_from {
    ($ti:ty, $to:ty) => {
        impl ExactIntCast<$ti> for $to {
            #[inline]
            fn exact_int_cast(v: $ti) -> Fallible<Self> {
                Ok(From::from(v))
            }
        }
    };
}

impl_exact_int_cast_from!(u32, u32);

pub trait InfCast<TI>: Sized {
    fn inf_cast(v: TI) -> Fallible<Self>;
    fn neg_inf_cast(v: TI) -> Fallible<Self>;
}

macro_rules! impl_inf_cast_exact {
    ($ti:ty, $to:ty) => {
        impl InfCast<$ti> for $to {
            fn inf_cast(v: $ti) -> Fallible<Self> {
                ExactIntCast::exact_int_cast(v)
            }
            fn neg_inf_cast(v: $ti) -> Fallible<Self> {
                ExactIntCast::exact_int_cast(v)
            }
        }
    };
}

impl_inf_cast_exact!(u32, u32);

pub trait AlertingMul: Sized {
    fn alerting_mul(&self, v: &Self) -> Fallible<Self>;
}

impl AlertingMul for u32 {
    #[inline]
    fn alerting_mul(&self, v: &Self) -> Fallible<Self> {
        <u32>::checked_mul(*self, *v).ok_or_else(|| {
            err!(
                FailedFunction,
                "{} * {} overflows. Consider tightening your parameters.",
                self,
                v
            )
        })
    }
}

pub trait InfMul: Sized + AlertingMul {
    fn inf_mul(&self, v: &Self) -> Fallible<Self>;
    fn neg_inf_mul(&self, v: &Self) -> Fallible<Self>;
}

impl InfMul for u32 {
    fn inf_mul(&self, other: &Self) -> Fallible<Self> {
        self.alerting_mul(other)
    }
    fn neg_inf_mul(&self, other: &Self) -> Fallible<Self> {
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

fn max_by<T, F: FnOnce(&T, &T) -> Fallible<Ordering>>(v1: T, v2: T, compare: F) -> Fallible<T> {
    compare(&v1, &v2).map(|cmp| match cmp {
        Ordering::Less | Ordering::Equal => v2,
        Ordering::Greater => v1,
    })
}

fn min_by<T, F: FnOnce(&T, &T) -> Fallible<Ordering>>(v1: T, v2: T, compare: F) -> Fallible<T> {
    compare(&v1, &v2).map(|cmp| match cmp {
        Ordering::Less | Ordering::Equal => v1,
        Ordering::Greater => v2,
    })
}

pub trait TotalOrd: PartialOrd + Sized {
    fn total_cmp(&self, other: &Self) -> Fallible<Ordering>;

    fn total_max(self, other: Self) -> Fallible<Self> {
        max_by(self, other, TotalOrd::total_cmp)
    }

    fn total_min(self, other: Self) -> Fallible<Self> {
        min_by(self, other, TotalOrd::total_cmp)
    }

    fn total_clamp(self, min: Self, max: Self) -> Fallible<Self> {
        if min > max {
            return fallible!(FailedFunction, "min cannot be greater than max");
        }
        Ok(if let Ordering::Less = self.total_cmp(&min)? {
            min
        } else if let Ordering::Greater = self.total_cmp(&max)? {
            max
        } else {
            self
        })
    }

    fn total_lt(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_lt())
    }

    fn total_le(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_le())
    }

    fn total_gt(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_gt())
    }

    fn total_ge(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_ge())
    }
}

impl TotalOrd for f64 {
    fn total_cmp(&self, other: &Self) -> Fallible<Ordering> {
        PartialOrd::partial_cmp(self, other).ok_or_else(|| crate::Error {
            variant: crate::ErrorVariant::FailedFunction,
            message: Some(("f64 cannot not be null when clamping.").to_string()),
        })
    }
}

impl TotalOrd for u32 {
    fn total_cmp(&self, other: &Self) -> Fallible<Ordering> {
        Ok(Ord::cmp(self, other))
    }
}

impl StabilityMap{
    pub fn new(map: impl Fn(&IntDistance) -> IntDistance + 'static) -> Self {
        StabilityMap(Rc::new(move |d_in: &IntDistance| Ok(map(d_in))))
    }
    pub fn new_fallible(map: impl Fn(&IntDistance) -> Fallible<IntDistance> + 'static) -> Self {
        StabilityMap(Rc::new(map))
    }
    pub fn new_from_constant(c: IntDistance) -> Self
    where
    IntDistance: Clone,
    IntDistance: DistanceConstant<IntDistance>,
    {
        StabilityMap::new_fallible(move |d_in: &IntDistance| {
            if c <IntDistance::zero() {
                return fallible!(FailedMap, "constant must be non-negative");
            }
            IntDistance::inf_cast(d_in.clone())?.inf_mul(&c)
        })
    }
    pub fn eval(&self, input_distance: &IntDistance) -> Fallible<IntDistance> {
        (self.0)(input_distance)
    }
}

impl StabilityMap {
    pub fn make_chain<MX: 'static + Metric>(
        map1: &StabilityMap,
        map0: &StabilityMap,
    ) -> Self {
        let map1 = map1.0.clone();
        let map0 = map0.0.clone();
        StabilityMap(Rc::new(move |d_in: &IntDistance| map1(&map0(d_in)?)))
    }
}

pub trait DistanceConstant<TI>: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

impl<TI, TO> DistanceConstant<TI> for TO where TO: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

pub trait Metric: Default + Clone + PartialEq + Debug {
    type Distance;
}

#[derive(Clone)]
pub struct Transformation {
    pub input_domain: VectorDomain<AtomDomain>,
    pub output_domain: VectorDomain<AtomDomain>,
    pub function: Function,
    pub input_metric: SymmetricDistance,
    pub output_metric: SymmetricDistance,
    pub stability_map: StabilityMap,
}

impl Transformation
where
    (VectorDomain<AtomDomain>, SymmetricDistance): MetricSpace,
    (VectorDomain<AtomDomain>, SymmetricDistance): MetricSpace,
{
    pub fn new(
        input_domain: VectorDomain<AtomDomain>,
        output_domain: VectorDomain<AtomDomain>,
        function: Function,
        input_metric: SymmetricDistance,
        output_metric: SymmetricDistance,
        stability_map: StabilityMap,
    ) -> Fallible<Self> {
        (input_domain.clone(), input_metric.clone()).assert_compatible()?;
        (output_domain.clone(), output_metric.clone()).assert_compatible()?;
        Ok(Self {
            input_domain,
            output_domain,
            function,
            input_metric,
            output_metric,
            stability_map,
        })
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
    fn member(&self, val: &Self::Carrier) -> Fallible<bool>;
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
    value: &Vec<f64>,
    row_function: &impl Fn(&f64) -> Fallible<f64>,
) -> Fallible<Vec<f64>> {
    value.iter().map(row_function).collect()
}

pub type IntDistance = u32;

pub trait MetricSpace {
    fn check(&self) -> bool;
    fn assert_compatible(&self) -> Fallible<()> {
        if !self.check() {
            fallible!(FailedFunction, "metric and domain are not compatible")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub variant: ErrorVariant,
    pub message: Option<String>,
    //pub backtrace: _Backtrace,
}

pub trait DatasetMetric: Metric<Distance = IntDistance> {
    const ORDERED: bool;
    const SIZED: bool;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Bounds {
    lower: Bound<f64>,
    upper: Bound<f64>,
}

impl Bounds {
    pub fn new_closed(bounds: (f64, f64)) -> Fallible<Self> {
        Self::new((Bound::Included(bounds.0), Bound::Included(bounds.1)))
    }
    /// Checks that the arguments are well-formed.
    pub fn new(bounds: (Bound<f64>, Bound<f64>)) -> Fallible<Self> {
        let (lower, upper) = bounds;
        fn get<T>(value: &Bound<T>) -> Option<&T> {
            match value {
                Bound::Included(value) => Some(value),
                Bound::Excluded(value) => Some(value),
                Bound::Unbounded => None,
            }
        }
        if let Some((v_lower, v_upper)) = get(&lower).zip(get(&upper)) {
            if v_lower > v_upper {
                return fallible!(
                    MakeDomain,
                    "lower bound may not be greater than upper bound"
                );
            }
            if v_lower == v_upper {
                match (&lower, &upper) {
                    (Bound::Included(_l), Bound::Excluded(_u)) => {
                        return fallible!(MakeDomain, "upper bound excludes inclusive lower bound")
                    }
                    (Bound::Excluded(_l), Bound::Included(_u)) => {
                        return fallible!(MakeDomain, "lower bound excludes inclusive upper bound")
                    }
                    _ => (),
                }
            }
        }
        Ok(Bounds { lower, upper })
    }
    pub fn lower(&self) -> Option<&f64> {
        match &self.lower {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None,
        }
    }
    pub fn upper(&self) -> Option<&f64> {
        match &self.upper {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None,
        }
    }
}

impl Bounds {
    pub fn member(&self, val: &f64) -> Fallible<bool> {
        Ok(match &self.lower {
            Bound::Included(bound) => val.total_ge(bound)?,
            Bound::Excluded(bound) => val.total_gt(bound)?,
            Bound::Unbounded => true,
        } && match &self.upper {
            Bound::Included(bound) => val.total_le(bound)?,
            Bound::Excluded(bound) => val.total_lt(bound)?,
            Bound::Unbounded => true,
        })
    }
}

pub trait CheckNull {
    fn is_null(&self) -> bool;
}

pub trait CheckAtom: CheckNull + Sized + Clone + PartialEq + Debug {
    fn is_bounded(&self, _bounds: Bounds) -> Fallible<bool> {
        fallible!(FailedFunction, "bounds check is not implemented")
    }
    fn check_member(&self, bounds: Option<Bounds>, nullable: bool) -> Fallible<bool> {
        if let Some(bounds) = bounds {
            if !self.is_bounded(bounds)? {
                return Ok(false);
            }
        }
        if !nullable && self.is_null() {
            return Ok(false);
        }
        Ok(true)
    }
}

impl CheckNull for f64 {
    #[inline]
    fn is_null(&self) -> bool {
        self.is_nan()
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

impl CheckAtom for f64 {
    fn is_bounded(&self, bounds: Bounds) -> Fallible<bool> {
        bounds.member(self)
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
    type Carrier = f64;
    fn member(&self, val: &Self::Carrier) -> Fallible<bool> {
        val.check_member(self.bounds.clone(), self.nullable)
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
            return fallible!(FailedFunction, "Domain has null values");
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
    fn member(&self, val: &Self::Carrier) -> Fallible<bool> {
        for e in val {
            if !self.element_domain.member(e)? {
                return Ok(false);
            }
        }
        if let Some(size) = self.size {
            if size != val.len() {
                return Ok(false);
            }
        }
        Ok(true)
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
    type Distance = IntDistance;
}

impl<D: Domain> MetricSpace for (VectorDomain<D>, SymmetricDistance) {
    fn check(&self) -> bool {
        true
    }
}

impl DatasetMetric for SymmetricDistance {
    const ORDERED: bool = false;
    const SIZED: bool = false;
}

pub type Fallible<T> = Result<T, Error>;

pub(crate) fn make_row_by_row_fallible(
    input_domain: VectorDomain<AtomDomain>,
    input_metric: SymmetricDistance,
    output_row_domain: AtomDomain,
    row_function: impl 'static + Fn(&f64) -> Fallible<f64>,
) -> Fallible<Transformation> {
    let output_domain = translate(&input_domain, output_row_domain);
    Transformation::new(
        input_domain,
        output_domain,
        Function::new_fallible(move |arg: &Vec<f64>| apply_rows(arg, &row_function)),
        input_metric.clone(),
        input_metric,
        StabilityMap::new_from_constant(1),
    )
}

pub fn make_clamp(
    input_domain: VectorDomain<AtomDomain>,
    input_metric: SymmetricDistance,
    bounds: (f64, f64),
) -> Fallible<Transformation>
where
    (VectorDomain<AtomDomain>, SymmetricDistance): MetricSpace,
{
    input_domain.element_domain.assert_non_null()?;

    let mut output_row_domain = input_domain.element_domain.clone();
    output_row_domain.bounds = Some(Bounds::new_closed(bounds.clone())?);

    make_row_by_row_fallible(
        input_domain,
        input_metric,
        output_row_domain,
        move |arg: &f64| arg.clone().total_clamp(bounds.0.clone(), bounds.1.clone()),
    )
}

fn clamp_transform() -> Fallible<Transformation> {
    let id = VectorDomain::new(AtomDomain::default());
    make_clamp(id, SymmetricDistance, (10.0, 20.0))
}

fn example_client() -> Fallible<()> {
    let _count = //(
        clamp_transform()?;
    // >> then_sum() >> then_laplace(20.0))?;

    // println!("privacy spend {:?}", count.map(&1));

    // let res: f64 = count.invoke(&vec![1., 2., 3.0f64])?;

    // println!("{:?}", res);

    Ok(())
}

fn main() {
    example_client().unwrap();
}
