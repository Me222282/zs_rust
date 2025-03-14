use zs_core::Radian;
use zs_core::Float;
use zs_macros::{generate_matrix, generate_matrix_multiply};

use crate::real;
use crate::{Vector2, Vector3, Vector4};

#[generate_matrix(2, Vector2)]
#[matrix_square]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2 {}
#[generate_matrix(3, Vector3)]
#[matrix_square]
#[matrix_constructors(scale, 3, Vector3, 2, Vector2)]
#[matrix_constructors(trans, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix3 {}
#[generate_matrix(4, Vector4)]
#[matrix_square]
#[matrix_constructors(scale, 4, Vector4, 3, Vector3, 2, Vector2)]
#[matrix_constructors(trans, Vector3)]
#[matrix_constructors(trans, 2, 2, Vector2, Vector2)]
#[matrix_constructors(rotate, 3)]
pub struct Matrix4 {}

#[generate_matrix(2, 3, Vector2, Vector3)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2x3 {}
#[generate_matrix(2, 4, Vector2, Vector4)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix2x4 {}

#[generate_matrix(3, 2, Vector3, Vector2)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix3x2 {}
#[generate_matrix(3, 4, Vector3, Vector4)]
#[matrix_constructors(scale, 3, Vector3, 2, Vector2)]
#[matrix_constructors(trans, Vector3, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix3x4 {}

#[generate_matrix(4, 2, Vector4, Vector2)]
#[matrix_constructors(scale, Vector2)]
#[matrix_constructors(rotate)]
pub struct Matrix4x2 {}
#[generate_matrix(4, 3, Vector4, Vector3)]
#[matrix_constructors(scale, 3, Vector3, 2, Vector2)]
#[matrix_constructors(trans, Vector2, Vector3)]
#[matrix_constructors(rotate)]
pub struct Matrix4x3 {}

generate_matrix_multiply!(2, Matrix2);
generate_matrix_multiply!(2, Matrix2, Matrix2x3, 3, Matrix2x3);
generate_matrix_multiply!(2, Matrix2, Matrix2x4, 4, Matrix2x4);
generate_matrix_multiply!(3, Matrix3);
generate_matrix_multiply!(3, Matrix3, Matrix3x2, 2, Matrix3x2);
generate_matrix_multiply!(3, Matrix3, Matrix3x4, 4, Matrix3x4);
generate_matrix_multiply!(4, Matrix4);
generate_matrix_multiply!(4, Matrix4, Matrix4x2, 2, Matrix4x2);
generate_matrix_multiply!(4, Matrix4, Matrix4x3, 3, Matrix4x3);

generate_matrix_multiply!(2, Matrix2x3, Matrix3x2, 2, Matrix2);
generate_matrix_multiply!(2, Matrix2x3, Matrix3, 3);
generate_matrix_multiply!(2, Matrix2x3, Matrix3x4, 4, Matrix2x4);
generate_matrix_multiply!(2, Matrix2x4, Matrix4x2, 2, Matrix2);
generate_matrix_multiply!(2, Matrix2x4, Matrix4x3, 3, Matrix2x3);
generate_matrix_multiply!(2, Matrix2x4, Matrix4, 4);
generate_matrix_multiply!(3, Matrix3x2, Matrix2, 2);
generate_matrix_multiply!(3, Matrix3x2, Matrix2x3, 3, Matrix3);
generate_matrix_multiply!(3, Matrix3x2, Matrix2x4, 4, Matrix3x4);
generate_matrix_multiply!(3, Matrix3x4, Matrix4x2, 2, Matrix3x2);
generate_matrix_multiply!(3, Matrix3x4, Matrix4x3, 3, Matrix3);
generate_matrix_multiply!(3, Matrix3x4, Matrix4, 4);
generate_matrix_multiply!(4, Matrix4x2, Matrix2, 2, Matrix4x2);
generate_matrix_multiply!(4, Matrix4x2, Matrix2x3, 3, Matrix4x3);
generate_matrix_multiply!(4, Matrix4x2, Matrix2x4, 4, Matrix4);
generate_matrix_multiply!(4, Matrix4x3, Matrix3x2, 2, Matrix4x2);
generate_matrix_multiply!(4, Matrix4x3, Matrix3, 3);
generate_matrix_multiply!(4, Matrix4x3, Matrix3x4, 4, Matrix4);

pub type Mat2 = Matrix2<real>;
pub type Mat3 = Matrix3<real>;
pub type Mat4 = Matrix4<real>;

pub type Mat2x3 = Matrix2x3<real>;
pub type Mat2x4 = Matrix2x4<real>;
pub type Mat3x2 = Matrix3x2<real>;
pub type Mat3x4 = Matrix3x4<real>;
pub type Mat4x2 = Matrix4x2<real>;
pub type Mat4x3 = Matrix4x3<real>;

impl<S: Float + Copy> Matrix3<S>
    where Radian<S>: Into<S>
{
    pub fn create_rotation(mut axis: Vector3<S>, angle: Radian<S>) -> Matrix3<S>
    {
        // normalize and create a local copy of the vector.
        axis = axis.normalised();
        let axis_x = axis.x;
        let axis_y = axis.y; 
        let axis_z = axis.z;

        // calculate angles
        let cos: S = angle.into().cos();
        let sin: S = -angle.into().sin();
        let t = S::one() - cos;

        // do the conversion math once
        let t_xx = t * axis_x * axis_x;
        let t_xy = t * axis_x * axis_y;
        let t_xz = t * axis_x * axis_z;
        let t_yy = t * axis_y * axis_y;
        let t_yz = t * axis_y * axis_z;
        let t_zz = t * axis_z * axis_z;

        let sin_x = sin * axis_x;
        let sin_y = sin * axis_y;
        let sin_z = sin * axis_z;

        return [
            [t_xx + cos, t_xy - sin_z, t_xz + sin_y],
            [t_xy + sin_z, t_yy + cos, t_yz - sin_x],
            [t_xz - sin_y, t_yz + sin_x, t_zz + cos]
        ].into();
    }
}

impl<S: Float + Copy> Matrix3x4<S>
    where Radian<S>: Into<S>
{
    pub fn create_rotation(mut axis: Vector3<S>, angle: Radian<S>) -> Matrix3x4<S>
    {
        // normalize and create a local copy of the vector.
        axis = axis.normalised();
        let axis_x = axis.x;
        let axis_y = axis.y; 
        let axis_z = axis.z;

        // calculate angles
        let cos: S = angle.into().cos();
        let sin: S = -angle.into().sin();
        let t = S::one() - cos;

        // do the conversion math once
        let t_xx = t * axis_x * axis_x;
        let t_xy = t * axis_x * axis_y;
        let t_xz = t * axis_x * axis_z;
        let t_yy = t * axis_y * axis_y;
        let t_yz = t * axis_y * axis_z;
        let t_zz = t * axis_z * axis_z;

        let sin_x = sin * axis_x;
        let sin_y = sin * axis_y;
        let sin_z = sin * axis_z;

        return [
            [t_xx + cos, t_xy - sin_z, t_xz + sin_y, S::zero()],
            [t_xy + sin_z, t_yy + cos, t_yz - sin_x, S::zero()],
            [t_xz - sin_y, t_yz + sin_x, t_zz + cos, S::zero()]
        ].into();
    }
}

impl<S: Float + Copy> Matrix4x3<S>
    where Radian<S>: Into<S>
{
    pub fn create_rotation(mut axis: Vector3<S>, angle: Radian<S>) -> Matrix4x3<S>
    {
        // normalize and create a local copy of the vector.
        axis = axis.normalised();
        let axis_x = axis.x;
        let axis_y = axis.y; 
        let axis_z = axis.z;

        // calculate angles
        let cos: S = angle.into().cos();
        let sin: S = -angle.into().sin();
        let t = S::one() - cos;

        // do the conversion math once
        let t_xx = t * axis_x * axis_x;
        let t_xy = t * axis_x * axis_y;
        let t_xz = t * axis_x * axis_z;
        let t_yy = t * axis_y * axis_y;
        let t_yz = t * axis_y * axis_z;
        let t_zz = t * axis_z * axis_z;

        let sin_x = sin * axis_x;
        let sin_y = sin * axis_y;
        let sin_z = sin * axis_z;

        return [
            [t_xx + cos, t_xy - sin_z, t_xz + sin_y],
            [t_xy + sin_z, t_yy + cos, t_yz - sin_x],
            [t_xz - sin_y, t_yz + sin_x, t_zz + cos],
            [S::zero(), S::zero(), S::zero()]
        ].into();
    }
}

impl<S: Float + Copy> Matrix4<S>
    where Radian<S>: Into<S>
{
    pub fn create_rotation(mut axis: Vector3<S>, angle: Radian<S>) -> Matrix4<S>
    {
        // normalize and create a local copy of the vector.
        axis = axis.normalised();
        let axis_x = axis.x;
        let axis_y = axis.y; 
        let axis_z = axis.z;

        // calculate angles
        let cos: S = angle.into().cos();
        let sin: S = -angle.into().sin();
        let t = S::one() - cos;

        // do the conversion math once
        let t_xx = t * axis_x * axis_x;
        let t_xy = t * axis_x * axis_y;
        let t_xz = t * axis_x * axis_z;
        let t_yy = t * axis_y * axis_y;
        let t_yz = t * axis_y * axis_z;
        let t_zz = t * axis_z * axis_z;

        let sin_x = sin * axis_x;
        let sin_y = sin * axis_y;
        let sin_z = sin * axis_z;

        return [
            [t_xx + cos, t_xy - sin_z, t_xz + sin_y, S::zero()],
            [t_xy + sin_z, t_yy + cos, t_yz - sin_x, S::zero()],
            [t_xz - sin_y, t_yz + sin_x, t_zz + cos, S::zero()],
            [S::zero(), S::zero(), S::zero(), S::one()]
        ].into();
    }
    
    pub fn create_orthographic_off_centre(
        left: S, right: S, top: S, bottom: S, depth_near: S, depth_far: S) -> Self
    {
        let inv_rl = S::one() / (right - left);
        let inv_tb = S::one() / (top - bottom);
        let inv_fn = S::one() / (depth_far - depth_near);
        
        let z = S::zero();
        return [
            [inv_rl + inv_rl, z, z, z],
            [z, inv_tb + inv_tb, z, z],
            [z, z, -inv_fn - inv_fn, z],
            [
                -(right + left) * inv_rl,
                -(top + bottom) * inv_tb,
                -(depth_far + depth_near) * inv_fn,
                S::one()]
            ].into();
    }

    pub fn create_orthographic(width: S, height: S, depth_near: S, depth_far: S) -> Self
    {
        // small cost for getting 1 / 2
        let o = S::one();
        let h = o / (o + o);
        return Self::create_orthographic_off_centre(
            -width * h, width * h, height * h, -height * h, depth_near, depth_far);
    }

    pub fn create_perspective_off_centre(
        left: S, right: S, top: S, bottom: S, depth_near: S, depth_far: S) -> Self
    {
        // if (depth_near <= 0)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(depth_near));
        // }

        // if (depth_far <= 0)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(depth_far));
        // }

        // if (depth_near >= depth_far)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(depth_near));
        // }

        let width_multi = S::one() / (right - left);
        let height_multi = S::one() / (bottom - top);
        let depth_mutli = S::one() / (depth_far - depth_near);
        let near2 = depth_near + depth_near;
        
        let z = S::zero();
        return [
            [near2 * width_multi, z, z, z],
            [z, near2 * height_multi, z, z],
            [
                -(right + left) * width_multi,
                -(bottom + top) * height_multi,
                depth_far * depth_mutli,
                S::one()
            ],
            [z, z, -depth_far * depth_near * depth_mutli, z]
        ].into();
    }

    pub fn create_perspective_field_of_view(
        fovy: Radian<S>, aspect: S, depth_near: S, depth_far: S) -> Self
    {
        // if (fovy <= 0 || fovy > Math.PI)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(fovy));
        // }

        // if (aspect <= 0)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(aspect));
        // }

        // if (depth_near <= 0)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(depth_near));
        // }

        // if (depth_far <= 0)
        // {
        //     throw new ArgumentOutOfRangeException(nameof(depth_far));
        // }
        
        // small cost for getting 1 / 2
        let o = S::one();
        let h = o / (o + o);
        
        let depth_mutli = o / (depth_far - depth_near);
        let degree = (fovy.into() * h).tan();
        
        let z = S::zero();
        return [
            [o / (aspect * degree), z, z, z],
            [z, o / degree, z, z],
            [z, z, depth_far * depth_mutli, o],
            [z, z, -depth_far * depth_near * depth_mutli, z]
        ].into();
    }

    // pub fn look_at_v(eye: Vector3<S>, target: Vector3<S>, up: Vector3<S>) -> Self
    // {
    //     let z = (eye - target).normalised();
    //     let x = up.cross(z).normalised();
    //     let y = z.cross(x).normalised();

    //     return [
    //         [x.x, y.x, z.x, S::zero()],
    //         [x.y, y.y, z.y, S::zero()],
    //         [x.z, y.z, z.z, S::zero()],
    //         [
    //             -((x.x * eye.x) + (x.y * eye.y) + (x.z * eye.z)),
    //             -((y.x * eye.x) + (y.y * eye.y) + (y.z * eye.z)),
    //             -((z.x * eye.x) + (z.y * eye.y) + (z.z * eye.z)),
    //             S::one()]
    //         ].into();
    // }

    // pub fn look_at(
    //     eye_x: S, eye_y: S, eye_z: S, target_x: S, target_y: S, target_z: S, up_x: S, up_y: S, up_z: S) -> Self
    // {
    //     return Self::look_at_v(
    //         Vector3::new(eye_x, eye_y, eye_z),
    //         Vector3::new(target_x, target_y, target_z),
    //         Vector3::new(up_x, up_y, up_z));
    // }
}