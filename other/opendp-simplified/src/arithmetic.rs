
use crate::Fallible;
use crate::err;
use crate::fallible;
use crate::ExactIntCast;

/// Fallible absolute value that returns an error if overflowing.
///
/// This can return an error when a signed integer is the smallest negative value.
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingAbs;
/// assert!(i8::MIN.alerting_abs().is_err());
/// ```
pub trait AlertingAbs: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`, returns `Ok(out)` where $out = |self|$ or `Err(e)`.
    fn alerting_abs(&self) -> Fallible<Self>;
}

/// Fallible addition that returns an error if overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingAdd;
/// assert!(i8::MAX.alerting_add(&1).is_err());
/// ```
pub trait AlertingAdd: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `Ok(self + v)` if the result does not overflow, else `Err(e)`
    fn alerting_add(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible subtraction that returns an error if overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingSub;
/// assert!(i8::MIN.alerting_sub(&1).is_err());
/// ```
pub trait AlertingSub: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `Ok(self - v)` if the result does not overflow, else `Err(e)`
    fn alerting_sub(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible multiplication that returns an error if overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingMul;
/// assert!(i8::MAX.alerting_mul(&2).is_err());
/// ```
pub trait AlertingMul: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `Ok(self * v)` if the result does not overflow, else `Err(e)`
    fn alerting_mul(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible division that returns an error if overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingDiv;
/// assert!(1u8.alerting_div(&0).is_err());
/// ```
pub trait AlertingDiv: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `Ok(self / v)` if the result does not overflow, else `Err(e)`
    fn alerting_div(&self, v: &Self) -> Fallible<Self>;
}

/// Fallibly raise to the power.
///
/// Returns an error if overflowing.
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::AlertingPow;
/// assert!(2u8.alerting_pow(&8).is_err());
/// ```
pub trait AlertingPow: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `Ok(self^v)` if the result does not overflow, else `Err(e)`
    fn alerting_pow(&self, p: &Self) -> Fallible<Self>;
}

/// Addition that saturates at the numeric bounds instead of overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::SaturatingAdd;
/// assert_eq!(i8::MAX.saturating_add(i8::MAX), i8::MAX);
/// ```
pub trait SaturatingAdd: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `self + v`, saturating at the relevant high or low boundary of the type.
    fn saturating_add(&self, v: &Self) -> Self;
}

/// Multiplication that saturates at the numeric bounds instead of overflowing.
///
/// Avoids unrecoverable panics that could leak private information.
/// ```
/// use opendp::traits::SaturatingMul;
/// assert_eq!(i8::MAX.saturating_mul(2), i8::MAX);
/// ```
pub trait SaturatingMul: Sized {
    /// # Proof Definition
    /// For any `self` and `v` of type `Self`,
    /// returns `self * v`, saturating at the relevant high or low boundary of the type.
    fn saturating_mul(&self, v: &Self) -> Self;
}

/// Fallible exponentiation with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfExp: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_exp()` either returns `Ok(out)`,
    /// where $out \ge \exp(self)$, or `Err(e)`.
    fn inf_exp(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_exp()` either returns `Ok(out)`,
    /// where $out \le \exp(self)$, or `Err(e)`.
    fn neg_inf_exp(self) -> Fallible<Self>;
}

/// Fallible natural logarithm with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfLn: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_ln()` either returns `Ok(out)`,
    /// where $out \ge \ln(self)$, or `Err(e)`.
    fn inf_ln(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_ln()` either returns `Ok(out)`,
    /// where $out \le \ln(self)$, or `Err(e)`.
    fn neg_inf_ln(self) -> Fallible<Self>;
}

/// Fallible base-2 logarithm with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfLog2: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_log2()` either returns `Ok(out)`,
    /// where $out \ge \log_2(self)$, or `Err(e)`.
    fn inf_log2(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_log2()` either returns `Ok(out)`,
    /// where $out \le \log_2(self)$, or `Err(e)`.
    fn neg_inf_log2(self) -> Fallible<Self>;
}

/// Fallible square root with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfSqrt: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_log2()` either returns `Ok(out)`,
    /// where $out \ge \sqrt{self}$, or `Err(e)`.
    fn inf_sqrt(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_log2()` either returns `Ok(out)`,
    /// where $out \le \sqrt{self}$, or `Err(e)`.
    fn neg_inf_sqrt(self) -> Fallible<Self>;
}

/// Fallibly raise self to the power with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfPow: Sized + AlertingPow {
    /// # Proof Definition
    /// For any two values `self` and `p` of type `Self`,
    /// `self.inf_pow(p)` either returns `Ok(out)`,
    /// where $out \ge self^{p}$, or `Err(e)`.
    fn inf_pow(&self, p: &Self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any two values `self` and `p` of type `Self`,
    /// `self.neg_inf_pow(p)` either returns `Ok(out)`,
    /// where $out \le self^{p}$, or `Err(e)`.
    fn neg_inf_pow(&self, p: &Self) -> Fallible<Self>;
}

/// Fallible addition with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfAdd: Sized + AlertingAdd {
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.inf_add(v)` either returns `Ok(out)`,
    /// where $out \ge self + v$, or `Err(e)`.
    fn inf_add(&self, v: &Self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.neg_inf_add(v)` either returns `Ok(out)`,
    /// where $out \le self + v$, or `Err(e)`.
    fn neg_inf_add(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible subtraction with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfSub: Sized + AlertingSub {
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.inf_sub(v)` either returns `Ok(out)`,
    /// where $out \ge self - v$, or `Err(e)`.
    fn inf_sub(&self, v: &Self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.neg_inf_sub(v)` either returns `Ok(out)`,
    /// where $out \le self - v$, or `Err(e)`.
    fn neg_inf_sub(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible multiplication with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfMul: Sized + AlertingMul {
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.inf_mul(v)` either returns `Ok(out)`,
    /// where $out \ge self \cdot v$, or `Err(e)`.
    fn inf_mul(&self, v: &Self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.neg_inf_mul(v)` either returns `Ok(out)`,
    /// where $out \le self \cdot v$, or `Err(e)`.
    fn neg_inf_mul(&self, v: &Self) -> Fallible<Self>;
}

/// Fallible division with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
pub trait InfDiv: Sized + AlertingDiv {
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.inf_div(v)` either returns `Ok(out)`,
    /// where $out \ge self / v$, or `Err(e)`.
    fn inf_div(&self, v: &Self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any two values `self` and `v` of type `Self`,
    /// `self.neg_inf_div(v)` either returns `Ok(out)`,
    /// where $out \le self / v$, or `Err(e)`.
    fn neg_inf_div(&self, v: &Self) -> Fallible<Self>;
}

/// Fallibly exponentiate and subtract one with specified rounding.
///
/// Throws an error if the ideal output is not finite or representable.
/// This provides more numerical stability than computing the quantity outright.
pub trait InfExpM1: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_exp_m1()` either returns `Ok(out)`,
    /// where $out \ge \exp(self) - 1$, or `Err(e)`.
    fn inf_exp_m1(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_exp_m1()` either returns `Ok(out)`,
    /// where $out \le \exp(self) - 1$, or `Err(e)`.
    fn neg_inf_exp_m1(self) -> Fallible<Self>;
}

/// Fallible logarithm of the argument plus one with specified rounding.
pub trait InfLn1P: Sized {
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.inf_ln_1p()` either returns `Ok(out)`,
    /// where $out \ge \ln(self + 1)$, or `Err(e)`.
    fn inf_ln_1p(self) -> Fallible<Self>;
    /// # Proof Definition
    /// For any `self` of type `Self`,
    /// `self.neg_inf_ln_1p()` either returns `Ok(out)`,
    /// where $out \le \ln(self + 1)$, or `Err(e)`.
    fn neg_inf_ln_1p(self) -> Fallible<Self>;
}

// BEGIN IMPLEMENTATIONS