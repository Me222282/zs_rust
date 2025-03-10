use zs_macros::{generate_matrix, generate_vector};

#[generate_vector(7)]
#[mult_vec_args(Matrix7)]
pub struct Vector7 {}

#[generate_vector(6)]
pub struct Vector6 {}

#[generate_matrix(7, Vector7)]
#[matrix_square]
#[mult_mat_args]
#[matrix_constructors(Vector7, Vector6)]
pub struct Matrix7 {}