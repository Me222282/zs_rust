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

pub trait Vector<S>: Add<Output = Self> + Sub<Output = Self> + Mul<S, Output = Self> + Sized + Copy
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
    
    #[inline]
    fn length(self) -> S
        where S: Float
    {
        return self.squared_length().sqrt();
    }
    #[inline]
    fn distance(self, other: Self) -> S
        where S: Float
    {   
        return self.squared_distance(other).sqrt();
    }
    #[inline]
    fn normalised(self) -> Self
        where S: Copy + Float
    {
        let scale = S::one() / self.length();
        return self * scale;
    }
}