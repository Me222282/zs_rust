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