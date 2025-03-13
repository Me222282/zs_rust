use crate::*;

impl<S: Float> Float for Radian<S>
{
    fn nan() -> Self {
        Self(S::nan())
    }

    fn infinity() -> Self {
        Self(S::infinity())
    }

    fn neg_infinity() -> Self {
        Self(S::neg_infinity())
    }

    fn neg_zero() -> Self {
        Self(S::neg_zero())
    }

    fn min_value() -> Self {
        Self(S::min_value())
    }

    fn min_positive_value() -> Self {
        Self(S::min_positive_value())
    }

    fn max_value() -> Self {
        Self(S::max_value())
    }

    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.0.classify()
    }

    fn floor(self) -> Self {
        Self(self.0.floor())
    }

    fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    fn round(self) -> Self {
        Self(self.0.round())
    }

    fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    fn fract(self) -> Self {
        Self(self.0.fract())
    }

    fn abs(self) -> Self {
        Self(self.0.abs())
    }

    fn signum(self) -> Self {
        Self(self.0.signum())
    }

    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self(self.0.mul_add(a.0, b.0))
    }

    fn recip(self) -> Self {
        Self(self.0.recip())
    }

    fn powi(self, n: i32) -> Self {
        Self(self.0.powi(n))
    }

    fn powf(self, n: Self) -> Self {
        Self(self.0.powf(n.0))
    }

    fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    fn exp(self) -> Self {
        Self(self.0.exp())
    }

    fn exp2(self) -> Self {
        Self(self.0.exp2())
    }

    fn ln(self) -> Self {
        Self(self.0.ln())
    }

    fn log(self, base: Self) -> Self {
        Self(self.0.log(base.0))
    }

    fn log2(self) -> Self {
        Self(self.0.log2())
    }

    fn log10(self) -> Self {
        Self(self.0.log10())
    }

    fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    fn abs_sub(self, other: Self) -> Self {
        Self(self.0.abs_sub(other.0))
    }

    fn cbrt(self) -> Self {
        Self(self.0.cbrt())
    }

    fn hypot(self, other: Self) -> Self {
        Self(self.0.hypot(other.0))
    }

    fn sin(self) -> Self {
        Self(self.0.sin())
    }

    fn cos(self) -> Self {
        Self(self.0.cos())
    }

    fn tan(self) -> Self {
        Self(self.0.tan())
    }

    fn asin(self) -> Self {
        Self(self.0.asin())
    }

    fn acos(self) -> Self {
        Self(self.0.acos())
    }

    fn atan(self) -> Self {
        Self(self.0.atan())
    }

    fn atan2(self, other: Self) -> Self {
        Self(self.0.atan2(other.0))
    }

    fn sin_cos(self) -> (Self, Self) {
        let v = self.0.sin_cos();
        (Self(v.0), Self(v.1))
    }

    fn exp_m1(self) -> Self {
        Self(self.0.exp_m1())
    }

    fn ln_1p(self) -> Self {
        Self(self.0.ln_1p())
    }

    fn sinh(self) -> Self {
        Self(self.0.sinh())
    }

    fn cosh(self) -> Self {
        Self(self.0.cosh())
    }

    fn tanh(self) -> Self {
        Self(self.0.tanh())
    }

    fn asinh(self) -> Self {
        Self(self.0.asinh())
    }

    fn acosh(self) -> Self {
        Self(self.0.acosh())
    }

    fn atanh(self) -> Self {
        Self(self.0.atanh())
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}