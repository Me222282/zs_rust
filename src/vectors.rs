use std::ops::{Sub, Mul, Neg};

use zs_core::Radian;
use zs_macros::generate_vector;

use crate::real;

#[generate_vector(2)]
pub struct Vector2 {}
#[generate_vector(3)]
pub struct Vector3 {}
#[generate_vector(4)]
pub struct Vector4 {}

pub type Vector1<S> = S;

pub type Vec2 = Vector2<real>;
pub type Vec3 = Vector3<real>;
pub type Vec4 = Vector4<real>;
pub type Vec2I = Vector2<i32>;
pub type Vec3I = Vector3<i32>;
pub type Vec4I = Vector4<i32>;

impl<S: Neg<Output = S>> Vector2<S>
{
    pub fn rotated_90(self) -> Self
    {
        return Self
        {
            x: -self.y,
            y: self.x
        };
    }
    pub fn rotated_270(self) -> Self
    {
        return Self
        {
            x: self.y,
            y: -self.x
        };
    }
}
impl<S: zs_core::Float> Vector2<S>
    where Radian<S>: Into<S>
{
    pub fn rotated(self, angle: Radian<S>) -> Self
    {
        let a: S = angle.into();
        let sin = a.sin();
        let cos = a.cos();
        
        return Self
        {
            x: (self.x * cos) - (self.y * sin),
            y: (self.x * sin) + (self.y * cos)
        };
    }
    pub fn rotated_point(self, point: Self, angle: Radian<S>) -> Self
    {
        let a: S = angle.into();
        let sin = a.sin();
        let cos = a.cos();
        
        let vec = self - point;
        
        return Self
        {
            x: (vec.x * cos) - (vec.y * sin) + point.x,
            y: (vec.x * sin) + (vec.y * cos) + point.y
        };
    }
}
impl<S: Copy + Sub<Output = S> + Mul<Output = S>> Vector2<S>
{
    pub fn perp_dot(self, other: Self) -> S
    {
        return self.x * other.y - self.y * other.x; 
    }
}