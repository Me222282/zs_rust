use std::ops::{Mul, Neg, Sub};

use zs_core::*;

use crate::Vector2;

#[derive(Clone, Copy, PartialEq)]
pub struct Line2<S>
{
    pub dir: Vector2<S>,
    pub loc: Vector2<S>
}
impl<S: Copy> Line2<S>
{
    #[inline]
    pub fn new(dir: Vector2<S>, loc: Vector2<S>) -> Self
    {
        return Self { dir, loc };
    }
    #[inline]
    pub fn new_s(dir_x: S, dir_y: S, loc_x: S, loc_y: S) -> Self
    {
        return Self {
            dir: Vector2::new(dir_x, dir_y),
            loc: Vector2::new(loc_x, loc_y)
        };
    }
    
    #[inline]
    pub fn normal(self) -> Vector2<S>
        where S: Neg<Output = S>
    {
        return self.dir.rotated_90();
    }
    
    #[inline]
    pub fn normalised(self) -> Self
        where S: Float
    {
        return Self { dir: self.dir.normalised(), loc: self.loc };
    }
    
    pub fn intersects(self, other: Self) -> Option<Vector2<S>>
        where S: NumOps + Zero
    {
        let b = self.dir;
        let d = other.dir;

        let p_dot = b.perp_dot(d);

        // If b dot d == 0, it means the lines are parallel and have an intersection of infinity
        if p_dot.is_zero() { return None; }

        let c = other.loc - self.loc;
        let t = c.perp_dot(d) / p_dot;

        return Some(self.loc + (b * t));
    }
    
    pub fn reflect(self, x: Vector2<S>) -> Vector2<S>
        where S: NumOps
    {
        let t = (x - self.loc).dot(self.dir) / self.dir.squared_length();
        let q = self.loc + (self.dir * t);
        return q + q - x;
    }
    
    pub fn project(self, x: Vector2<S>) -> Vector2<S>
        where S: NumOps
    {
        let t = (x - self.loc).dot(self.dir) / self.dir.squared_length();
        return self.loc + (self.dir * t);
    }
    
    pub fn reflect_line(self, other: Self) -> Option<Self>
        where S: NumOps + Zero
    {
        let np = self.reflect(other.loc);
        let i = self.intersects(other);
        return match i {
            Some(inter) => Some(Self { dir: np - inter, loc: inter}),
            None => None
        };
    }
    
    #[inline]
    pub fn perpendicular(self) -> Self
        where S: Neg<Output = S>
    {
        let l = self.loc;
        return Self { dir: self.normal(), loc: l };
    }
    
    pub fn distance_from_point(self, point: Vector2<S>) -> S
        where S: Float
    {
        let n = self.dir.rotated_90();

        let sl = n.squared_length();
        let v = (point - self.loc).dot(n);

        if sl.is_one() { return v; }

        return v / sl.sqrt();
    }
    
    pub fn squared_distance_from_point(self, point: Vector2<S>) -> S
        where S: NumOps + One + Neg<Output = S> + PartialEq
    {
        let n = self.dir.rotated_90();

        let sl = n.squared_length();
        let v = (point - self.loc).dot(n);
        let v = v * v;

        if sl.is_one() { return v; }

        return v / sl;
    }
    
    #[inline]
    pub fn contains(self, point: Vector2<S>) -> bool
        where S: Sub<Output = S> + Mul<Output = S> + Zero
    {
        return (point - self.loc).perp_dot(self.dir).is_zero();
    }
    
    pub fn geometrically_equals(self, other: Self) -> bool
        where S: Sub<Output = S> + Mul<Output = S> + Zero + PartialEq
    {
        let c = self.dir.perp_dot(other.dir);
            
        if !c.is_zero() { return false; }
        
        let diff = self.loc - other.loc;
        
        return diff.is_zero() || diff.perp_dot(self.dir).is_zero();
    }
}