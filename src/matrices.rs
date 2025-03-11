use zs_core::Float;
use zs_macros::generate_matrix;

use crate::real;
use crate::{Vector2, Vector3, Vector4};

#[generate_matrix(2, Vector2)]
#[matrix_square]
#[matrix_mult]
#[matrix_mult(3, Matrix2x3, Matrix2x3)]
#[matrix_mult(4, Matrix2x4, Matrix2x4)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2 {}
#[generate_matrix(3, Vector3)]
#[matrix_square]
#[matrix_mult(2, Matrix3x2, Matrix3x2)]
#[matrix_mult]
#[matrix_mult(4, Matrix3x4, Matrix3x4)]
#[matrix_constructors(scale, Vector3)]
#[matrix_constructors(trans, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix3 {}
#[generate_matrix(4, Vector4)]
#[matrix_square]
#[matrix_mult(2, Matrix4x2, Matrix4x2)]
#[matrix_mult(3, Matrix4x3, Matrix4x3)]
#[matrix_mult]
#[matrix_constructors(scale, Vector4)]
#[matrix_constructors(trans, Vector3)]
#[matrix_constructors(rotate, 3)]
pub struct Matrix4 {}

#[generate_matrix(2, 3, Vector2, Vector3)]
#[matrix_mult(2, Matrix3x2, Matrix2)]
#[matrix_mult(3, Matrix3, Matrix2x3)]
#[matrix_mult(4, Matrix3x4, Matrix2x4)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2x3 {}
#[generate_matrix(2, 4, Vector2, Vector4)]
#[matrix_mult(2, Matrix4x2, Matrix2)]
#[matrix_mult(3, Matrix4x3, Matrix2x3)]
#[matrix_mult(4, Matrix4, Matrix2x4)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2x4 {}

#[generate_matrix(3, 2, Vector3, Vector2)]
#[matrix_mult(2, Matrix2, Matrix3x2)]
#[matrix_mult(3, Matrix2x3, Matrix3)]
#[matrix_mult(4, Matrix2x4, Matrix3x4)]
#[matrix_constructors(scale, Vector3)]
#[matrix_constructors(rotate)]
pub struct Matrix3x2 {}
#[generate_matrix(3, 4, Vector3, Vector4)]
#[matrix_mult(2, Matrix4x2, Matrix3x2)]
#[matrix_mult(3, Matrix4x3, Matrix3)]
#[matrix_mult(4, Matrix4, Matrix3x4)]
#[matrix_constructors(scale, Vector3)]
#[matrix_constructors(trans, Vector3, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix3x4 {}

#[generate_matrix(4, 2, Vector4, Vector2)]
#[matrix_mult(2, Matrix2, Matrix4x2)]
#[matrix_mult(3, Matrix2x3, Matrix4x3)]
#[matrix_mult(4, Matrix2x4, Matrix4)]
#[matrix_constructors(scale, Vector4)]
#[matrix_constructors(rotate)]
pub struct Matrix4x2 {}
#[generate_matrix(4, 3, Vector4, Vector3)]
#[matrix_mult(2, Matrix3x2, Matrix4x2)]
#[matrix_mult(3, Matrix3, Matrix4x3)]
#[matrix_mult(4, Matrix3x4, Matrix4)]
#[matrix_constructors(scale, Vector4)]
#[matrix_constructors(trans, Vector2, Vector3)]
#[matrix_constructors(rotate)]
pub struct Matrix4x3 {}

pub type Mat2 = Matrix2<real>;
pub type Mat3 = Matrix3<real>;
pub type Mat4 = Matrix4<real>;

pub type Mat2x3 = Matrix2x3<real>;
pub type Mat2x4 = Matrix2x4<real>;
pub type Mat3x2 = Matrix3x2<real>;
pub type Mat3x4 = Matrix3x4<real>;
pub type Mat4x2 = Matrix4x2<real>;
pub type Mat4x3 = Matrix4x3<real>;

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
