mod angles;

pub use num_traits::*;
use std::fmt::{Display, Formatter, Result};
use std::error::Error;
use std::ops::*;

pub use crate::angles::*;

#[derive(Debug, Copy, Clone)]
pub struct SliceToVectorError { }
impl Display for SliceToVectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        #[allow(deprecated)]
        return self.description().fmt(f);
    }
}
impl Error for SliceToVectorError {
    #[allow(deprecated)]
    fn description(&self) -> &str
    {
        "could not convert slice to vector"
    }
}

pub trait VectorInt<S>: Add<Output = Self> + Sub<Output = Self> + Mul<S, Output = Self> + Sized + Copy
{
    fn dot(self, other: Self) -> S;
    
    #[inline]
    fn squared_length(self) -> S
    {
        return self.dot(self);
    }
    #[inline]
    fn squared_distance(self, other: Self) -> S
    {
        return (other - self).squared_length();
    }
    fn lerp(self, other: Self, blend: S) -> Self
    {
        return ((other - self) * blend) + self;
    }
    #[inline]
    fn bary_centric(self, b: Self, c: Self, u: S, v: S) -> Self
    {
        return (self + ((b - self) * u)) + ((c - self) * v);
    }
}
pub trait VectorFloat<S>: VectorInt<S>
{
    fn length(self) -> S;
    fn distance(self, other: Self) -> S;
    fn normalised(self) -> Self;
}

impl<S: Float, T: VectorInt<S>> VectorFloat<S> for T
{
    #[inline]
    fn length(self) -> S
    {
        return self.squared_length().sqrt();
    }
    #[inline]
    fn distance(self, other: Self) -> S
    {   
        return self.squared_distance(other).sqrt();
    }
    #[inline]
    fn normalised(self) -> Self
        where S: Copy
    {
        let scale = S::one() / self.length();
        return self * scale;
    }
}