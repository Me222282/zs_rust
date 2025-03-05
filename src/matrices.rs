use zs_core::Float;
use zs_macros::generate_matrix_square;

use crate::real;
use crate::{Vector2, Vector3, Vector4};

#[generate_matrix_square(2)]
pub struct Matrix2 {}
#[generate_matrix_square(3)]
pub struct Matrix3 {}
#[generate_matrix_square(4)]
pub struct Matrix4 {}

pub type Mat2 = Matrix2<real>;
pub type Mat3 = Matrix3<real>;
pub type Mat4 = Matrix4<real>;

impl<S: Float> Matrix2<S>
{
    pub fn create_rotation(angle: S) -> Self
    {
        let cos = angle.cos();
        let sin = angle.sin();

        return [[cos, sin], [-sin, cos]].into();
    }

    pub fn create_scale(scale: S) -> Self
    {
        return [[scale, S::zero()], [S::zero(), scale]].into();
    }

    pub fn create_scale_2(scale_x: S, scale_y: S) -> Self
    {
        return [[scale_x, S::zero()], [S::zero(), scale_y]].into();
    }

    pub fn create_scale_v(scale: Vector2<S>) -> Self
    {
        return Self::create_scale_2(scale.x, scale.y);
    }
}