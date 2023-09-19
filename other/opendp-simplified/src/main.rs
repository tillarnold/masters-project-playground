use prusti_contracts::*;
use std::{
    cmp::Ordering,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    ops::{Add, Bound},
    rc::Rc,
};

mod arithmetic;
use arithmetic::*;

#[macro_export]
macro_rules! fallible {
    ($variant:ident) => (Err(err!($variant)));
    ($variant:ident, $($inner:expr),+) => (Err(err!($variant, $($inner),+)));
}
// "error" is shadowed, and breaks intellij macro resolution
/// Create an instance of [`Error`]
#[macro_export]
macro_rules! err {
    // error without message
    ($variant:ident) => ($crate::Error {
        variant: $crate::ErrorVariant::$variant,
        //message: None,
    });
    // error with explicit message
    ($variant:ident, $message:expr) => ($crate::Error {
        variant: $crate::ErrorVariant::$variant,
        //message: Some($message.to_string()), // ToString is impl'ed for String
        //backtrace: err!(@backtrace)
    });
    // args to format into message
    ($variant:ident, $template:expr, $($args:expr),+) =>
        (err!($variant, format!($template, $($args,)+)));

    (@backtrace) => (std::backtrace::Backtrace::capture());
}

fn main() {}

pub trait TotalOrd: PartialOrd + Sized {
    /// # Proof Definition
    /// For any two values `self` and `other` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if either `self` or `other` are null.
    /// Otherwise `out` is the [`Ordering`] of `self` and `other` as defined by [`PartialOrd`].
    fn total_cmp(&self, other: &Self) -> Fallible<Ordering>;

    /// # Proof Definition
    /// For any two values `self` and `other` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if either `self` or `other` are null.
    /// Otherwise returns `Some(out)` where `out` is the greater of `self` and `other` as defined by [`PartialOrd`].
    fn total_max(self, other: Self) -> Fallible<Self> {
        TotalOrd::total_cmp(&self, &other).map(|cmp| match cmp {
            Ordering::Less | Ordering::Equal => other,
            Ordering::Greater => self,
        })
    }

    /// # Proof Definition
    /// For any two values `self` and `other` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if either `self` or `other` are null.
    /// Otherwise returns `Some(out)` where `out` is the lesser of `self` and `other` as defined by [`PartialOrd`].
    fn total_min(self, other: Self) -> Fallible<Self> {
        TotalOrd::total_cmp(&self, &other).map(|cmp| match cmp {
            Ordering::Less | Ordering::Equal => self,
            Ordering::Greater => other,
        })
    }

    /// # Proof Definition
    /// For any three values `self`, `min` and `max` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if any of `self`, `min` or `max` are null.
    /// Otherwise returns `Some(out)` where `out` is `min` if $self \lt min$, `max` if $self \gt max$, or else `self`.
    fn total_clamp(self, min: Self, max: Self) -> Fallible<Self> {
        if min > max {
            return Err(Error {
                variant: ErrorVariant::FailedFunction,
                //message: Some("min cannot be greater than max".to_string()),
                // backtrace: std::backtrace::Backtrace::capture(),
            });
        }
        Ok(if let Ordering::Less = self.total_cmp(&min)? {
            min
        } else if let Ordering::Greater = self.total_cmp(&max)? {
            max
        } else {
            self
        })
    }

    /// # Proof Definition
    /// For any two values `self` and `other` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if either `self` or `other` are null.
    /// Otherwise returns `Ok(out)` where `out` is true if $self \lt other$.
    fn total_lt(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_lt())
    }

    /// # Proof Definition
    /// For any two values `self` and `other` of type `Self`, returns `Ok(out)` or `Err(e)`.
    /// The implementation returns `Err(e)` if either `self` or `other` are null.
    /// Otherwise returns `Ok(out)` where `out` is true if $self \le other$.
    fn total_le(&self, other: &Self) -> Fallible<bool> {
        Ok(self.total_cmp(other)?.is_le())
    }
}

pub trait ExactIntBounds {
    /// # Proof Definition
    /// `Self::MAX_CONSECUTIVE` is the largest integer-consecutive finite value that can be represented by `Self`.
    const MAX_CONSECUTIVE: Self;
    /// # Proof Definition
    /// `Self::MIN_CONSECUTIVE` is the smallest integer-consecutive finite value that can be represented by `Self`.
    const MIN_CONSECUTIVE: Self;
}

pub trait ExactIntCast<TI>: Sized + ExactIntBounds {
    /// # Proof Definition
    /// For any `v` of type `TI`, `Self::exact_int_cast(value)` either
    /// returns `Err(e)` if `v` is smaller than `Self::MIN_CONSECUTIVE` or greater than `Self::MAX_CONSECUTIVE`,
    /// or `Ok(out)` where $out = v$.
    fn exact_int_cast(v: TI) -> Fallible<Self>;
}

macro_rules! impl_exact_int_bounds {
    ($($ty:ty),*) => ($(impl ExactIntBounds for $ty {
        const MAX_CONSECUTIVE: Self = Self::MAX;
        const MIN_CONSECUTIVE: Self = Self::MIN;
    })*)
}
impl_exact_int_bounds!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl ExactIntCast<u32> for u32 {
    #[inline]
    fn exact_int_cast(v: u32) -> Fallible<Self> {
        Ok(From::from(v))
    }
}
pub trait InfCast<TI>: Sized {
    /// # Proof Definition
    /// For any `v` of type `TI`, `Self::inf_cast(value)` either returns `Err(e)`,
    /// or `Ok(out)` where $out \ge v$.
    fn inf_cast(v: TI) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `v` of type `TI`, `Self::inf_cast(value)` either returns `Err(e)`,
    /// or `Ok(out)` where $out \le v$.
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

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;

    /// Sets `self` to the additive identity element of `Self`, `0`.
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }

    /// Returns `true` if `self` is equal to the additive identity.
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

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

pub trait DistanceConstant<TI>: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}
impl<TI, TO> DistanceConstant<TI> for TO where TO: 'static + InfCast<TI> + InfMul + TotalOrd + Zero {}

#[derive(PartialEq, Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Error {
    pub variant: ErrorVariant,
    //pub message: Option<String>,
    //pub backtrace: _Backtrace,
}

pub type Fallible<T> = Result<T, Error>;

pub trait Domain: Clone + PartialEq + Debug {
    type Carrier;

    //TODO: is it ok to ignore the error here

    #[pure]
    fn member(&self, val: &Self::Carrier) -> Fallible<bool>;
}

pub trait DatasetDomain: Domain {
    type ElementDomain: Domain;
}

pub struct Function<TI, TO> {
    _pd1: PhantomData<TI>,
    _pd2: PhantomData<TO>,
}

impl<TI, TO> Function<TI, TO> {
    fn new() -> Self {
        Function {
            _pd1: PhantomData,
            _pd2: PhantomData,
        }
    }

    #[pure]
    pub fn eval(&self, arg: &TI) -> Fallible<TO>
    where
        TO: Copy,
    {
        fallible!(NotImplemented)
    }
}

// pub struct Function<TI, TO> {
//     pub function: Rc<dyn Fn(&TI) -> Fallible<TO>>,
// }

// impl<TI, TO> Function<TI, TO> {
//     pub fn new(function: impl Fn(&TI) -> TO + 'static) -> Self {
//         Self::new_fallible(move |arg| Ok(function(arg)))
//     }

//     pub fn new_fallible(function: impl Fn(&TI) -> Fallible<TO> + 'static) -> Self {
//         Self {
//             function: Rc::new(function),
//         }
//     }

//     pub fn eval(&self, arg: &TI) -> Fallible<TO> {
//         (self.function)(arg)
//     }
// }

pub struct StabilityMap<MI: Metric, MO: Metric> {
    res: MO::Distance,
    _pd: PhantomData<MI>,
}

impl<MI: Metric, MO: Metric> StabilityMap<MI, MO>
where
    MO::Distance: Clone,
{
    pub fn new(res: MO::Distance) -> Self {
        StabilityMap {
            _pd: PhantomData,
            res: res,
        }
    }

    pub fn new_from_constant(c: MO::Distance) -> Self {
        StabilityMap::new(c)
    }
    #[pure]
    pub fn eval(&self, input_distance: &MI::Distance) -> Fallible<MO::Distance> {
        Ok(self.res.clone())
    }
}

pub trait Metric: Default + Clone + PartialEq + Debug {
    type Distance;
}

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

pub trait RowByRowDomain<DO: DatasetDomain>: DatasetDomain {
    fn translate(&self, output_row_domain: DO::ElementDomain) -> DO;
    fn apply_rows(
        value: &Self::Carrier,
        row_function: &impl Fn(
            &<Self::ElementDomain as Domain>::Carrier,
        ) -> Fallible<<DO::ElementDomain as Domain>::Carrier>,
    ) -> Fallible<DO::Carrier>;
}

pub type IntDistance = u32;

pub trait DatasetMetric: Metric<Distance = IntDistance> {}

pub trait MetricSpace {
    fn check(&self) -> bool;
    fn assert_compatible(&self) -> Fallible<()> {
        if !self.check() {
            Err(Error {
                variant: ErrorVariant::FailedFunction,
                //message: Some("metric and domain are not compatible".to_string()),
                // backtrace: std::backtrace::Backtrace::capture(),
            })
        } else {
            Ok(())
        }
    }
}

pub trait CheckNull {
    /// # Proof Definition
    /// For any `value` of type `Self`, returns true if is null, otherwise false.
    fn is_null(&self) -> bool;
}

#[derive(Clone, PartialEq)]
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
        // fn get<T>(value: &Bound<T>) -> Option<&T> {
        //     match value {
        //         Bound::Included(value) => Some(value),
        //         Bound::Excluded(value) => Some(value),
        //         Bound::Unbounded => None,
        //     }
        // }
        // if let Some((v_lower, v_upper)) = get(&lower).zip(get(&upper)) {
        //     if v_lower > v_upper {
        //         return fallible!(
        //             MakeDomain,
        //             "lower bound may not be greater than upper bound"
        //         );
        //     }
        //     if v_lower == v_upper {
        //         match (&lower, &upper) {
        //             (Bound::Included(_l), Bound::Excluded(_u)) => {
        //                 return fallible!(MakeDomain, "upper bound excludes inclusive lower bound")
        //             }
        //             (Bound::Excluded(_l), Bound::Included(_u)) => {
        //                 return fallible!(MakeDomain, "lower bound excludes inclusive upper bound")
        //             }
        //             _ => (),
        //         }
        //     }
        // }
        Ok(Bounds { lower, upper })
    }
    // pub fn lower(&self) -> Option<&T> {
    //     match &self.lower {
    //         Bound::Included(v) => Some(v),
    //         Bound::Excluded(v) => Some(v),
    //         Bound::Unbounded => None,
    //     }
    // }
    // pub fn upper(&self) -> Option<&T> {
    //     match &self.upper {
    //         Bound::Included(v) => Some(v),
    //         Bound::Excluded(v) => Some(v),
    //         Bound::Unbounded => None,
    //     }
    // }
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

predicate! {
    fn are_close_under<D1, D2, M: Metric>(d1: &D1, d2: &D2, distance: &M::Distance, mertic: &M) -> bool {
        // How do we do this?
        //  metric.distance(d1, d2) <= distance
        true
    }
}

predicate! {
    fn valid_transform<DI: Domain, DO: Domain, M>
        (a: &Transformation<DI, DO, M, M>) -> bool
    where
    DI: Domain,
    DO: Domain,
    M: DatasetMetric<Distance = IntDistance>,

    {
      forall (|el : DI::Carrier|
            a.input_domain.member(&el) === Ok(true) ==>
            match a.function.eval(&el) {
                Err(_) => true,
                Ok(m) => a.output_domain.member(&m) === Ok(true)
            }
        )
        
        &&

        forall(
            |
              u: DI::Carrier,
              v : DI::Carrier,
              d_in: M::Distance,
              d_out: M::Distance
            |
            (a.input_domain.member(&u) === Ok(true) &&
             a.input_domain.member(&v) === Ok(true) &&
             are_close_under(&u,&v, &d_in, &a.input_metric) &&
             match a.stability_map.eval(&d_in) {
                Ok(stab) => stab <= d_out,
                Err(_) => false,
             }
            )
                ==> match (a.function.eval(&u), a.function.eval(&v)) {
                    (Ok(um), Ok(vm)) =>
                        are_close_under(&um, &vm, &d_out, &a.output_metric),
                    _ => false
                }



        )
    }
}

#[ensures(
    match result {
        Err(_) => true,
        Ok(t) => valid_transform(&t)
    }
)]
pub fn make_row_by_row_fallible<DI, DO, M>(
    input_domain: DI,
    input_metric: M,
    output_row_domain: DO::ElementDomain,
    row_function: (),
) -> Fallible<Transformation<DI, DO, M, M>>
where
    DI: RowByRowDomain<DO>,
    DO: DatasetDomain,
    M: DatasetMetric<Distance = IntDistance>,
    (DI, M): MetricSpace,
    (DO, M): MetricSpace,
    DO::Carrier: Copy,
    DI::Carrier: Copy, // needed for purity
                       // needed for purity
{
    let output_domain = input_domain.translate(output_row_domain);
    Transformation::new(
        input_domain,
        output_domain,
        Function::new(),
        input_metric.clone(),
        input_metric,
        StabilityMap::new_from_constant(1),
    )
}
