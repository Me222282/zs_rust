use zs_core::Float;
use zs_macros::{generate_matrix_square, MatMult};

use crate::real;
use crate::{Vector2, Vector3, Vector4};

#[generate_matrix_square(2, Vector2)]
#[derive(MatMult)]
#[mult_args(2, 2, Matrix2, Matrix2)]
pub struct Matrix2 {}
#[generate_matrix_square(3, Vector3)]
#[derive(MatMult)]
#[mult_args(3, 3, Matrix3, Matrix3)]
pub struct Matrix3 {}
#[generate_matrix_square(4, Vector4)]
#[derive(MatMult)]
#[mult_args(4, 4, Matrix4, Matrix4)]
pub struct Matrix4 {}

pub type Mat2 = Matrix2<real>;
pub type Mat3 = Matrix3<real>;
pub type Mat4 = Matrix4<real>;

impl<S: Float> Matrix2<S>
{
    #[inline]
    pub fn create_rotation(angle: S) -> Self
    {
        let cos = angle.cos();
        let sin = angle.sin();

        return [[cos, sin], [-sin, cos]].into();
    }
}

