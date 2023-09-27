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

pub struct Function<TI, TO> {
    pub function: Rc<dyn Fn(&TI) -> Fallible<TO>>,
}
impl<TI, TO> Clone for Function<TI, TO> {
    fn clone(&self) -> Self {
        Function {
            function: self.function.clone(),
        }
    }
}

impl<TI, TO> Function<TI, TO> {
    pub fn new(function: impl Fn(&TI) -> TO + 'static) -> Self {
        Self::new_fallible(move |arg| Ok(function(arg)))
    }

    pub fn new_fallible(function: impl Fn(&TI) -> Fallible<TO> + 'static) -> Self {
        Self {
            function: Rc::new(function),
        }
    }

    pub fn eval(&self, arg: &TI) -> Fallible<TO> {
        (self.function)(arg)
    }
}

impl<TI: 'static, TO: 'static> Function<TI, TO> {
    pub fn make_chain<TX: 'static>(
        function1: &Function<TX, TO>,
        function0: &Function<TI, TX>,
    ) -> Function<TI, TO> {
        let function0 = function0.function.clone();
        let function1 = function1.function.clone();
        Self::new_fallible(move |arg| function1(&function0(arg)?))
    }
}

pub struct StabilityMap<MI: Metric, MO: Metric>(
    pub Rc<dyn Fn(&MI::Distance) -> Fallible<MO::Distance>>,
);

impl<MI: Metric, MO: Metric> Clone for StabilityMap<MI, MO> {
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
impl_exact_int_bounds!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

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

impl<MI: Metric, MO: Metric> StabilityMap<MI, MO> {
    pub fn new(map: impl Fn(&MI::Distance) -> MO::Distance + 'static) -> Self {
        StabilityMap(Rc::new(move |d_in: &MI::Distance| Ok(map(d_in))))
    }
    pub fn new_fallible(map: impl Fn(&MI::Distance) -> Fallible<MO::Distance> + 'static) -> Self {
        StabilityMap(Rc::new(map))
    }
    pub fn new_from_constant(c: MO::Distance) -> Self
    where
        MI::Distance: Clone,
        MO::Distance: DistanceConstant<MI::Distance>,
    {
        StabilityMap::new_fallible(move |d_in: &MI::Distance| {
            if c < MO::Distance::zero() {
                return fallible!(FailedMap, "constant must be non-negative");
            }
            MO::Distance::inf_cast(d_in.clone())?.inf_mul(&c)
        })
    }
    pub fn eval(&self, input_distance: &MI::Distance) -> Fallible<MO::Distance> {
        (self.0)(input_distance)
    }
}

impl<MI: 'static + Metric, MO: 'static + Metric> StabilityMap<MI, MO> {
    pub fn make_chain<MX: 'static + Metric>(
        map1: &StabilityMap<MX, MO>,
        map0: &StabilityMap<MI, MX>,
    ) -> Self {
        let map1 = map1.0.clone();
        let map0 = map0.0.clone();
        StabilityMap(Rc::new(move |d_in: &MI::Distance| map1(&map0(d_in)?)))
    }
}

pub trait DistanceConstant<TI>: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

impl<TI, TO> DistanceConstant<TI> for TO where TO: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

pub trait Metric: Default + Clone + PartialEq + Debug {
    type Distance;
}

#[derive(Clone)]
pub struct Transformation<DI: Domain, DO: Domain, MI: Metric, MO: Metric> {
    pub input_domain: DI,
    pub output_domain: DO,
    pub function: Function<DI::Carrier, DO::Carrier>,
    pub input_metric: MI,
    pub output_metric: MO,
    pub stability_map: StabilityMap<MI, MO>,
}

impl<DI: Domain, DO: Domain, MI: Metric, MO: Metric> Transformation<DI, DO, MI, MO>
where
    (DI, MI): MetricSpace,
    (DO, MO): MetricSpace,
{
    pub fn new(
        input_domain: DI,
        output_domain: DO,
        function: Function<DI::Carrier, DO::Carrier>,
        input_metric: MI,
        output_metric: MO,
        stability_map: StabilityMap<MI, MO>,
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

pub trait RowByRowDomain<DO: DatasetDomain>: DatasetDomain {
    fn translate(&self, output_row_domain: DO::ElementDomain) -> DO;
    fn apply_rows(
        value: &Self::Carrier,
        row_function: &impl Fn(
            &<Self::ElementDomain as Domain>::Carrier,
        ) -> Fallible<<DO::ElementDomain as Domain>::Carrier>,
    ) -> Fallible<DO::Carrier>;
}

impl<DIA: Domain, DOA: Domain> RowByRowDomain<VectorDomain<DOA>> for VectorDomain<DIA> {
    fn translate(
        &self,
        output_row_domain: <VectorDomain<DOA> as DatasetDomain>::ElementDomain,
    ) -> VectorDomain<DOA> {
        VectorDomain {
            element_domain: output_row_domain,
            size: self.size,
        }
    }

    fn apply_rows(
        value: &Self::Carrier,
        row_function: &impl Fn(&DIA::Carrier) -> Fallible<DOA::Carrier>,
    ) -> Fallible<Vec<DOA::Carrier>> {
        value.iter().map(row_function).collect()
    }
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
pub struct Bounds<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<T: TotalOrd> Bounds<T> {
    pub fn new_closed(bounds: (T, T)) -> Fallible<Self> {
        Self::new((Bound::Included(bounds.0), Bound::Included(bounds.1)))
    }
    /// Checks that the arguments are well-formed.
    pub fn new(bounds: (Bound<T>, Bound<T>)) -> Fallible<Self> {
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
    pub fn lower(&self) -> Option<&T> {
        match &self.lower {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None,
        }
    }
    pub fn upper(&self) -> Option<&T> {
        match &self.upper {
            Bound::Included(v) => Some(v),
            Bound::Excluded(v) => Some(v),
            Bound::Unbounded => None,
        }
    }
}

impl<T: Clone + TotalOrd> Bounds<T> {
    pub fn member(&self, val: &T) -> Fallible<bool> {
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
    fn is_bounded(&self, _bounds: Bounds<Self>) -> Fallible<bool> {
        fallible!(FailedFunction, "bounds check is not implemented")
    }
    fn check_member(&self, bounds: Option<Bounds<Self>>, nullable: bool) -> Fallible<bool> {
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
    fn is_bounded(&self, bounds: Bounds<Self>) -> Fallible<bool> {
        bounds.member(self)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AtomDomain<T: CheckAtom> {
    pub bounds: Option<Bounds<T>>,
    nullable: bool,
}

impl<T: CheckAtom> Default for AtomDomain<T> {
    fn default() -> Self {
        AtomDomain {
            bounds: None,
            nullable: false,
        }
    }
}

impl<T: CheckAtom> Domain for AtomDomain<T> {
    type Carrier = T;
    fn member(&self, val: &Self::Carrier) -> Fallible<bool> {
        val.check_member(self.bounds.clone(), self.nullable)
    }
}

impl<T: CheckAtom> AtomDomain<T> {
    pub fn new(bounds: Option<Bounds<T>>, nullable: Option<Null<T>>) -> Self {
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
    pub fn bounds(&self) -> Option<&Bounds<T>> {
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

pub(crate) fn make_row_by_row_fallible<DI, DO, M>(
    input_domain: DI,
    input_metric: M,
    output_row_domain: DO::ElementDomain,
    row_function: impl 'static
        + Fn(
            &<DI::ElementDomain as Domain>::Carrier,
        ) -> Fallible<<DO::ElementDomain as Domain>::Carrier>,
) -> Fallible<Transformation<DI, DO, M, M>>
where
    DI: RowByRowDomain<DO>,
    DO: DatasetDomain,
    M: DatasetMetric<Distance = IntDistance>,
    (DI, M): MetricSpace,
    (DO, M): MetricSpace,
{
    let output_domain = input_domain.translate(output_row_domain);
    Transformation::new(
        input_domain,
        output_domain,
        Function::new_fallible(move |arg: &DI::Carrier| DI::apply_rows(arg, &row_function)),
        input_metric.clone(),
        input_metric,
        StabilityMap::new_from_constant(1),
    )
}

pub fn make_clamp<TA: 'static + Clone + TotalOrd + CheckAtom, M: DatasetMetric>(
    input_domain: VectorDomain<AtomDomain<TA>>,
    input_metric: M,
    bounds: (TA, TA),
) -> Fallible<Transformation<VectorDomain<AtomDomain<TA>>, VectorDomain<AtomDomain<TA>>, M, M>>
where
    (VectorDomain<AtomDomain<TA>>, M): MetricSpace,
{
    input_domain.element_domain.assert_non_null()?;

    let mut output_row_domain = input_domain.element_domain.clone();
    output_row_domain.bounds = Some(Bounds::<TA>::new_closed(bounds.clone())?);

    make_row_by_row_fallible(
        input_domain,
        input_metric,
        output_row_domain,
        move |arg: &TA| arg.clone().total_clamp(bounds.0.clone(), bounds.1.clone()),
    )
}

fn clamp_transform() -> Fallible<
    Transformation<
        VectorDomain<AtomDomain<f64>>,
        VectorDomain<AtomDomain<f64>>,
        SymmetricDistance,
        SymmetricDistance,
    >,
> {
    make_clamp(
        VectorDomain::new(AtomDomain::default()),
        SymmetricDistance,
        (10.0, 20.0),
    )
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
