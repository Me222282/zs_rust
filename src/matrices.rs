use zs_core::Float;
use zs_macros::generate_matrix;

use crate::real;
use crate::{Vector2, Vector3, Vector4};

#[generate_matrix(2, Vector2)]
#[matrix_square]
#[mult_mat_args]
#[matrix_constructors(scale, Vector2)]
// #[mult_args(2, 3, Matrix2x3, Matrix2x3)]
pub struct Matrix2 {}
#[generate_matrix(3, Vector3)]
#[matrix_square]
#[mult_mat_args]
#[matrix_constructors(scale, Vector3)]
#[matrix_constructors(trans, Vector2)]
pub struct Matrix3 {}
#[generate_matrix(4, Vector4)]
#[matrix_square]
#[mult_mat_args]
#[matrix_constructors(scale, Vector4)]
#[matrix_constructors(trans, Vector3)]
pub struct Matrix4 {}

// #[generate_matrix(2, 3, Vector2, Vector3)]
// #[derive(MatMult)]
// #[mult_args(2, 3, Matrix3, Matrix2x3)]
// pub struct Matrix2x3 {}

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
