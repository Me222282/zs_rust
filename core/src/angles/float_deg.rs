use crate::*;

impl<S: Float> Float for Degree<S>
{
    #[inline]
    fn nan() -> Self {
        Self(S::nan())
    }
    #[inline]
    fn infinity() -> Self {
        Self(S::infinity())
    }
    #[inline]
    fn neg_infinity() -> Self {
        Self(S::neg_infinity())
    }
    #[inline]
    fn neg_zero() -> Self {
        Self(S::neg_zero())
    }
    #[inline]
    fn min_value() -> Self {
        Self(S::min_value())
    }
    #[inline]
    fn min_positive_value() -> Self {
        Self(S::min_positive_value())
    }
    #[inline]
    fn max_value() -> Self {
        Self(S::max_value())
    }
    #[inline]
    fn is_nan(self) -> bool {
        self.0.is_nan()
    }
    #[inline]
    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }
    #[inline]
    fn is_finite(self) -> bool {
        self.0.is_finite()
    }
    #[inline]
    fn is_normal(self) -> bool {
        self.0.is_normal()
    }
    #[inline]
    fn classify(self) -> std::num::FpCategory {
        self.0.classify()
    }
    #[inline]
    fn floor(self) -> Self {
        Self(self.0.floor())
    }
    #[inline]
    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }
    #[inline]
    fn round(self) -> Self {
        Self(self.0.round())
    }
    #[inline]
    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }
    #[inline]
    fn fract(self) -> Self {
        Self(self.0.fract())
    }
    #[inline]
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
    #[inline]
    fn signum(self) -> Self {
        Self(self.0.signum())
    }
    #[inline]
    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }
    #[inline]
    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }
    #[inline]
    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }
    #[inline]
    fn recip(self) -> Self {
        Self(self.0.recip())
    }
    #[inline]
    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }
    #[inline]
    fn powf(self, n: Self) -> Self {
        Self(self.0.powf(n.0))
    }
    #[inline]
    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }
    #[inline]
    fn exp(self) -> Self {
        Self(self.0.exp())
    }
    #[inline]
    fn exp2(self) -> Self {
        Self(self.0.exp2())
    }
    #[inline]
    fn ln(self) -> Self {
        Self(self.0.ln())
    }
    #[inline]
    fn log(self, base: Self) -> Self {
        Self(self.0.log(base.0))
    }
    #[inline]
    fn log2(self) -> Self {
        Self(self.0.log2())
    }
    #[inline]
    fn log10(self) -> Self {
        Self(self.0.log10())
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
    #[inline]
    fn abs_sub(self, other: Self) -> Self {
        Self(self.0.abs_sub(other.0))
    }
    #[inline]
    fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }
    #[inline]
    fn hypot(self, other: Self) -> Self {
        Self(self.0.hypot(other.0))
    }
    #[inline]
    fn sin(self) -> Self {
        Self(self.0.sin())
    }
    #[inline]
    fn cos(self) -> Self {
        Self(self.0.cos())
    }
    #[inline]
    fn tan(self) -> Self {
        Self(self.0.tan())
    }
    #[inline]
    fn asin(self) -> Self {
        Self(self.0.asin())
    }
    #[inline]
    fn acos(self) -> Self {
        Self(self.0.acos())
    }
    #[inline]
    fn atan(self) -> Self {
        Self(self.0.atan())
    }
    #[inline]
    fn atan2(self, other: Self) -> Self {
        Self(self.0.atan2(other.0))
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let v = self.0.sin_cos();
        (Self(v.0), Self(v.1))
    }
    #[inline]
    fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
    }
    #[inline]
    fn ln_1p(self) -> Self {
        Self(self.0.ln_1p())
    }
    #[inline]
    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }
    #[inline]
    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }
    #[inline]
    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }
    #[inline]
    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }
    #[inline]
    fn acosh(self) -> Self {
        Self(self.0.acosh())
    }
    #[inline]
    fn atanh(self) -> Self {
        Self(self.0.atanh())
    }
    #[inline]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}