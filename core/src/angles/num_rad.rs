use std::ops::*;
use std::result::Result;

use crate::*;

impl<S: ToPrimitive> ToPrimitive for Radian<S>
{
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }
    #[inline]
    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
    #[inline]
    fn to_f32(&self) -> Option<f32>
    {
        self.0.to_f32()
    }
    #[inline]
    fn to_f64(&self) -> Option<f64>
    {
        self.0.to_f64()
    }
}

impl<S: NumCast> NumCast for Radian<S>
{
    #[inline]
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        let v = S::from(n);
        match v
        {
            Some(a) => Some(Self(a)),
            None => None
        }
    }
}

impl<S: Zero> Zero for Radian<S> {
    #[inline]
    fn zero() -> Self {
        Self(S::zero())
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
impl<S: One + PartialEq> One for Radian<S> {
    #[inline]
    fn one() -> Self {
        Self(S::one())
    }
    #[inline]
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<S: Add<Output = S>> Add for Radian<S> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl<S: Sub<Output = S>> Sub for Radian<S> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl<S: Mul<Output = S>> Mul for Radian<S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl<S: Div<Output = S>> Div for Radian<S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl<S: Rem<Output = S>> Rem for Radian<S> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}
impl<S: Neg<Output = S>> Neg for Radian<S> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl<S: Add<Output = S>> Add<S> for Radian<S> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: S) -> Self {
        Self(self.0 + rhs)
    }
}
impl<S: Sub<Output = S>> Sub<S> for Radian<S> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: S) -> Self {
        Self(self.0 - rhs)
    }
}
impl<S: Mul<Output = S>> Mul<S> for Radian<S> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: S) -> Self {
        Self(self.0 * rhs)
    }
}
impl<S: Div<Output = S>> Div<S> for Radian<S> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: S) -> Self {
        Self(self.0 / rhs)
    }
}
impl<S: Rem<Output = S>> Rem<S> for Radian<S> {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: S) -> Self {
        Self(self.0 % rhs)
    }
}

impl<S: Num> Num for Radian<S>
{
    type FromStrRadixErr = S::FromStrRadixErr;
    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, S::FromStrRadixErr> {
        let v = S::from_str_radix(str, radix);
        match v
        {
            Ok(a) => Ok(Self(a)),
            Err(e) => Err(e)
        }
    }
}